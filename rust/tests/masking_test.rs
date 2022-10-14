use std::sync::Arc;

use loader::masking;
use serde_yaml::Value;

#[tokio::main]
async fn basic_test(config:String) {
    let f = std::fs::File::open("tests/masking_tests.yaml").unwrap();
    let config_file:Value = serde_yaml::from_reader(f).unwrap();
    let config_ptr = Arc::new(config_file.get(config).unwrap().to_owned());

    masking::masking_top::run_main(config_ptr).await;
}

#[tokio::test]
async fn test_top() {
    let f = std::fs::File::open("tests/masking_tests.yaml").unwrap();
    let config_file:Value = serde_yaml::from_reader(f).unwrap();
    let config_ptr = Arc::new(config_file.get("basic").unwrap().to_owned());

    let result = masking::masking_top::run_main(config_ptr).await;
    assert!(result);
    //assert!(false)
    //std::process::exit(0);
}

#[test]
fn test_zmq_tcp_rust() {
    basic_test("zmq_tcp_rust".to_string());
}

#[test]
fn test_zmq_ipc() {
    basic_test("zmq_ipc".to_string());
}

#[test]
fn test_zmq_url() {
    basic_test("zmq_rust_url".to_string());
}


/* 
#[tokio::test]
async fn test_python_ipc() {
    let f = std::fs::File::open("tests/masking_tests.yaml").unwrap();
    let config_file:Value = serde_yaml::from_reader(f).unwrap();
    let _result = masking::masking_top::run_main(&config_file["zmq_ipc"]).await;
}
*/