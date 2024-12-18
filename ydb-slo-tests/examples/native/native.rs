extern crate ydb_slo_tests;

use crate::db::Db;
use clap::Parser;
use rand::Rng;
use ratelimit::Ratelimiter;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::timeout;
use tokio_util::task::TaskTracker;
use ydb_slo_tests::cli::{Command, SloTestsCli};
use ydb_slo_tests::generator::Generator;
use ydb_slo_tests::row::RowID;

mod db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let shutdown_signal = Arc::new(Notify::new());
    // let shutdown_signal_cloned = shutdown_signal.clone();
    // let ctrl_c_signal = tokio::signal::ctrl_c();

    let cli = SloTestsCli::parse();
    let command = cli.command.clone();

    // let signal_task = tokio::spawn(async move {
    //     let _ = ctrl_c_signal.await;
    //     shutdown_signal_cloned.notify_one();
    // });

    let db = Db::new(cli)
        .await
        .unwrap_or_else(|err| panic!("failed to initialize Ydb instance: {}", err));

    match command {
        Command::Create(create_args) => {
            db.create_table()
                .await
                .unwrap_or_else(|err| panic!("failed to create Ydb Table: {}", err));

            println!("Created table");

            // let generator = Generator::new(0);
            let tracker = TaskTracker::new();
            let db_ref = Arc::new(db.clone());
            let generator_ref = Arc::new(Mutex::new(Generator::new(0)));

            for _ in 0..create_args.initial_data_count {
                let db_ref = Arc::clone(&db_ref);
                let generator_ref = Arc::clone(&generator_ref);

                tracker.spawn(async move {
                    let db_ref = &db_ref;
                    // let mut generator_ref = &generator_ref;
                    let row = generator_ref.lock().await.generate();

                    db_ref.insert_row(row).await
                });
            }

            tracker.close();
            tracker.wait().await;

            println!("entries write ok");
        }
        Command::Cleanup => {
            db.drop_table()
                .await
                .unwrap_or_else(|err| panic!("failed to drop Ydb Table: {}", err));

            println!("Cleaned up table");
        }
        Command::Run(run_args) => {
            let db_ref = Arc::new(db.clone());
            // let workers = Workers::new(cli.clone(), storage.clone()).await?;
            // let worker_shutdown = shutdown_signal.clone();

            let tracker = TaskTracker::new();

            let read_rate_limiter_ref =
                Arc::new(Ratelimiter::builder(1, Duration::from_secs(run_args.read_rps)).build()?);

            for _ in 0..run_args.read_rps {
                let db_ref = Arc::clone(&db_ref);
                let read_rate_limiter_ref = Arc::clone(&read_rate_limiter_ref);

                timeout(
                    Duration::from_secs(run_args.time),
                    tracker.spawn(async move {
                        let db_ref = &db_ref;
                        let read_rate_limiter_ref = &read_rate_limiter_ref;

                        read_load(db_ref, read_rate_limiter_ref, run_args.initial_data_count).await;
                    }),
                )
                .await??;
            }

            let write_rate_limiter_ref =
                Arc::new(Ratelimiter::builder(1, Duration::from_secs(run_args.write_rps)).build()?);

            let generator_ref = Arc::new(Mutex::new(Generator::new(
                run_args.initial_data_count as RowID,
            )));

            for _ in 0..run_args.write_rps {
                let db_ref = Arc::clone(&db_ref);
                let write_rate_limiter_ref = Arc::clone(&write_rate_limiter_ref);
                let generator_ref = Arc::clone(&generator_ref);

                timeout(
                    Duration::from_secs(run_args.time),
                    tracker.spawn(async move {
                        let db_ref = &db_ref;
                        let write_rate_limiter_ref = &write_rate_limiter_ref;
                        let generator_ref = generator_ref.lock().await;

                        write_load(db_ref, write_rate_limiter_ref, &generator_ref).await;
                    }),
                )
                .await??
            }

            // let metrics_task = tokio::spawn(async move {
            //     workers.report_metrics(worker_shutdown).await.unwrap();
            // });

            tracker.close();
            tracker.wait().await;

            // let _ = tokio::try_join!(
            //     signal_task,
            //     join_all(read_tasks),
            //     join_all(write_tasks),
            //     // metrics_task
            // );

            println!("workers completed");
        }
    }

    println!("program finished");
    Ok(())
}

async fn read_load(db: &Db, limiter: &Ratelimiter, initial_data_count: u64) {
    loop {
        if let Err(_) = limiter.try_wait() {
            break;
        }

        let row_id = rand::thread_rng().gen_range(0..initial_data_count);
        let _ = db.read_row_by_id(row_id).await;
    }
}

async fn write_load(db: &Db, limiter: &Ratelimiter, generator: &Generator) {
    loop {
        if let Err(_) = limiter.try_wait() {
            break;
        }

        let row = generator.to_owned().generate();

        if let Err(err) = db.clone().insert_row(row).await {
            println!("write failed: `{}'", err);
        }
    }
}
