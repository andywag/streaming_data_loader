use std::sync::Arc;

use loader::datasets::{masking, squad};
use serde_yaml::Value;

#[tokio::main]
async fn basic_test(file:String, config:String, masking:bool) {
    let f = std::fs::File::open(file).unwrap();
    let config_file:Value = serde_yaml::from_reader(f).unwrap();
    let config_ptr = Arc::new(config_file.get(config).unwrap().to_owned());

    if masking {
        masking::masking_runner::run(config_ptr).await;
    }
    else {
        squad::squad_runner::run(config_ptr).await;
        //squad::squad_top::run_main(config_ptr).await;
    }
}

#[tokio::test]
async fn test_top() {
    let f = std::fs::File::open("tests/masking_tests.yaml").unwrap();
    let config_file:Value = serde_yaml::from_reader(f).unwrap();
    let config_ptr = Arc::new(config_file.get("basic").unwrap().to_owned());

    let result = masking::masking_runner::run(config_ptr).await;
    assert!(result);
    //assert!(false)
    //std::process::exit(0);
}

#[test]
fn test_zmq_tcp_rust() {
    basic_test("tests/masking_tests.yaml".to_string(),"zmq_tcp_rust".to_string(), true);
}

#[test]
fn test_zmq_ipc() {
    basic_test("tests/masking_tests.yaml".to_string(),"zmq_ipc".to_string(), true);
}

// Disable to issue with the url not being found
/* 
#[test]
fn test_zmq_url() {
    basic_test("tests/masking_tests.yaml".to_string(),"zmq_rust_url".to_string(), true);
}
*/

#[test]
fn test_basic_squad() {
    basic_test("tests/squad_tests.yaml".to_string(),"basic".to_string(), false);
}

#[test]
fn test_hugging_squad() {
    basic_test("tests/squad_tests.yaml".to_string(),"basic_hugging".to_string(), false);
}

/* 
#[tokio::test]
async fn test_python_ipc() {
    let f = std::fs::File::open("tests/masking_tests.yaml").unwrap();
    let config_file:Value = serde_yaml::from_reader(f).unwrap();
    let _result = masking::masking_top::run_main(&config_file["zmq_ipc"]).await;
}
*/