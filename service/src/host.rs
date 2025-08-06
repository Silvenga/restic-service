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

        info!("Stopped the service host successfully.");

        0
    }

    async fn run_with_config(config: ServiceConfiguration, cancellation_token: &CancellationToken) {
        let (sender, mut receiver) = channel::<(String, ResticJob)>(256);
        let job_manager_ref = Arc::new(JobManager::new(config, sender));

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

                info!("Job worker is stopped.");
            }
        });

        let server_task = task::spawn({
            let job_manager_ref = job_manager_ref.clone();
            let cancellation_token = cancellation_token.clone();
            async move {
                run_server(&job_manager_ref, &cancellation_token).await.unwrap();
            }
        });

        let main_task = task::spawn({
            let job_manager_ref = job_manager_ref.clone();
            let cancellation_token = cancellation_token.clone();
            async move {
                info!("Setting up {} jobs...", job_manager_ref.get_jobs().len());
                for (job_name, job_config) in job_manager_ref.get_jobs() {
                    info!(
                        "Scheduling job '{job_name}' with cron: '{}'.",
                        job_config.cron
                    );

                    let job = Job::cron(&format!("0 {}", job_config.cron)).unwrap();
                    scheduler
                        .insert(job, {
                            let jobs_manager = job_manager_ref.clone();
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
                info!("Cancellation triggered, stopping jobs...");
                // Drops the scheduler...
            }
        });

        main_task.await.unwrap();
        jobs_task.await.unwrap();
        cron_task.await.unwrap();
        server_task.await.unwrap();
    }
}
