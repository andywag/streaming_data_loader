
use pyo3::prelude::*;
use std::sync::mpsc::SyncSender;
use std::{thread, sync::mpsc};
use std::time::Duration;

use crate::{logger, config, tasks::cases::BasicCases};

#[tokio::main]
async fn start_process(tx:SyncSender<String>) {
    let example = BasicCases::Bert.get_config(true);
    let result = crate::tasks::run(example, config::TaskType::Mlm, None, None).await;
    log::info!("Final Result {}", result);
    thread::sleep(Duration::from_millis(10000));

    let _ = tx.send("Here I am".to_string());
}

fn test_int() -> usize {
    32
}
/// Formats the sum of two numbers as string.
#[pyfunction]
pub fn test(py: Python) -> usize {
    logger::create_logger();
    let (tx, rx) = mpsc::sync_channel::<String>(8);
    thread::spawn(|| {
        start_process(tx);
        /*for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(5000));
        }*/
    });
    let data = rx.recv();
    log::info!("Here.... {:?}", data);
    py.allow_threads(|| test_int())
}