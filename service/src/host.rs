use crate::config::{ResticJob, ServiceConfiguration, ServiceConfigurationManager};
use crate::jobs::JobRunner;
use async_cron_scheduler::{Job, Scheduler};
use chrono::Local;
use log::{info, warn};
use std::ffi::OsString;
use tokio::sync::mpsc::channel;
use tokio::task;
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;

pub struct ServiceHost {}

impl ServiceHost {
    pub async fn run(_arguments: Vec<OsString>, cancellation_token: &CancellationToken) -> u8 {
        let watcher = ServiceConfigurationManager::new()
            .watch_configuration()
            .await
            .unwrap();

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
                    if let Some((job_name, job_config)) = receiver.recv().await {
                        info!("Job '{job_name}' is running.");
                        let start = Instant::now();

                        JobRunner::run(&job_config, &cancellation_token).await;

                        info!(
                            "Job '{job_name}' is stopped after running for {:?}.",
                            start.elapsed()
                        );
                    }
                }
            }
        });

        let main_task = async move {
            // Move the scheduler into the main task.
            for (job_name, job_config) in config.jobs {
                info!(
                    "Scheduling job '{job_name}' with cron: '{}'.",
                    job_config.cron
                );

                let job = Job::cron(&format!("0 {}", job_config.cron)).unwrap();
                scheduler
                    .insert(job, {
                        let job_config = job_config.clone();
                        let sender = sender.clone();
                        move |_| {
                            info!("Job '{job_name}' is queued.");
                            sender
                                .try_send((job_name.clone(), job_config.clone()))
                                .unwrap();
                        }
                    })
                    .await;
            }
            cancellation_token.cancelled().await;
            info!("Stopping jobs...");
            // Drops the scheduler...
        };

        main_task.await;
        jobs_task.await.unwrap();
        cron_task.await.unwrap();
    }
}
