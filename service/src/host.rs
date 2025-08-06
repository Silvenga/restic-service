use crate::api::run_server;
use crate::config::{ResticJob, ServiceConfiguration, ServiceConfigurationManager};
use crate::jobs::{JobManager, JobRunner};
use async_cron_scheduler::{Job, Scheduler};
use chrono::Local;
use log::{info, warn};
use std::ffi::OsString;
use std::sync::Arc;
use tokio::runtime::Handle;
use tokio::sync::mpsc::channel;
use tokio::task;
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;

pub struct ServiceHost;

impl ServiceHost {
    pub async fn run(_arguments: Vec<OsString>, cancellation_token: &CancellationToken) -> u8 {
        let watcher = ServiceConfigurationManager::new()
            .watch_configuration()
            .await
            .expect("configuration file must exist to watch");

        while !cancellation_token.is_cancelled() {
            let configuration_cancellation_token = &cancellation_token.child_token();
            watcher.register_cancellation_token(configuration_cancellation_token);

            match watcher.read_configuration().await {
                Ok(config) => Self::run_with_config(config, configuration_cancellation_token).await,
                Err(e) => {
                    warn!("Configuration error, waiting for next update... Error: {e:?}");
                    configuration_cancellation_token.cancelled().await;
                }
            };
        }
        0
    }

    async fn run_with_config(config: ServiceConfiguration, cancellation_token: &CancellationToken) {
        info!("Starting {} jobs...", config.jobs.len());

        let (sender, mut receiver) = channel::<(String, ResticJob)>(256);
        let (mut scheduler, sched_service) = Scheduler::<Local>::launch(tokio::time::sleep);

        let cron_task = task::spawn(sched_service);

        let jobs_task = task::spawn({
            let cancellation_token = cancellation_token.clone();
            async move {
                while !cancellation_token.is_cancelled() {
                    tokio::select! {
                        job = receiver.recv() => {
                            if let Some((job_name, job_config)) = job {
                                info!("Job '{job_name}' is running.");
                                let start = Instant::now();

                                JobRunner::run(&job_config, &cancellation_token).await;

                                info!(
                                    "Job '{job_name}' is stopped after running for {:?}.",
                                    start.elapsed()
                                );
                            }
                        }
                        _ = cancellation_token.cancelled() => { }
                    }
                }
            }
        });

        let job_manager = Arc::new(JobManager::new(config, sender));

        let main_task = {
            let job_manager = job_manager.clone();
            async move {
                // Move the scheduler into the main task.
                for (job_name, job_config) in job_manager.get_jobs() {
                    info!(
                        "Scheduling job '{job_name}' with cron: '{}'.",
                        job_config.cron
                    );

                    let job = Job::cron(&format!("0 {}", job_config.cron)).unwrap();
                    scheduler
                        .insert(job, {
                            let jobs_manager = job_manager.clone();
                            move |_| {
                                let handle = Handle::current();
                                handle.block_on(async {
                                    match jobs_manager.queue_job(job_name.clone()).await {
                                        Ok(_) => (),
                                        Err(_) => {
                                            warn!("Failed to queue job '{job_name}' for execution.")
                                        }
                                    };
                                });
                            }
                        })
                        .await;
                }
                cancellation_token.cancelled().await;
                info!("Stopping jobs...");
                // Drops the scheduler...
            }
        };

        run_server(&job_manager, cancellation_token).await.unwrap();

        main_task.await;
        jobs_task.await.unwrap();
        cron_task.await.unwrap();
    }
}
