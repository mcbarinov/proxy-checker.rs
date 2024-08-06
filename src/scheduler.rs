use std::{sync::Arc, time::Duration};

use clokwerk::{AsyncScheduler, TimeUnits};

use crate::App;

pub fn run_scheduler(app: Arc<App>) {
    let mut scheduler = AsyncScheduler::new();

    let app_job = app.clone();
    scheduler.every(6.seconds()).run(move || {
        let app = app_job.clone();
        async move {
            if let Err(err) = app.source_service.next_check().await {
                tracing::error!("scheduler error / source_service.check_next / {:?}", err);
            }
        }
    });

    let app_job = app.clone();
    scheduler.every(2.seconds()).run(move || {
        let app = app_job.clone();
        async move {
            if let Err(err) = app.proxy_service.next_check().await {
                tracing::error!("scheduler error / proxy_service.check_next / {:?}", err);
            }
        }
    });

    tokio::spawn(async move {
        loop {
            scheduler.run_pending().await;
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    });
}
