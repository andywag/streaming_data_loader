use std::sync::Arc;

use loader::tasks::{masking, squad, multi_label, single_class};
use serde_yaml::Value;

enum TestType {
    _MASK,
    SQUAD,
    MULTI,
    SINGLE
}

#[tokio::main]

async fn basic_test(test_type:TestType, config:String) {
    loader::create_logger();

    let path = match test_type {
        TestType::_MASK =>  "tests/masking_tests.yaml",
        TestType::SQUAD => "tests/squad_tests.yaml",
        TestType::MULTI => "tests/multi_label.yaml",
        TestType::SINGLE => "tests/single_class.yaml",

    };
    
    let f = std::fs::File::open(path).unwrap();
    let config_file:Value = serde_yaml::from_reader(f).unwrap();
    let config_ptr = Arc::new(config_file.get(config).unwrap().to_owned());

    let result = match test_type {
        TestType::_MASK =>  masking::masking_runner::run(config_ptr).await,
        TestType::SQUAD => squad::runner::run(config_ptr).await,
        TestType::MULTI => multi_label::runner::run(config_ptr).await,
        TestType::SINGLE => single_class::runner::run(config_ptr).await,

    };
    log::info!("Result {}", result);
    assert!(result);
   
}

#[tokio::test]
async fn test_masking() {
    let f = std::fs::File::open("tests/masking_tests.yaml").unwrap();
    let config_file:Value = serde_yaml::from_reader(f).unwrap();
    let config_ptr = Arc::new(config_file.get("basic").unwrap().to_owned());

    let result = masking::masking_runner::run(config_ptr).await;
    assert!(result);
}

/* 
#[test]
fn test_mask_rust() {
    basic_test("tests/masking_tests.yaml".to_string(),"zmq_tcp_rust".to_string(), true);
}

#[test]
fn test_mask_python() {
    basic_test("tests/masking_tests.yaml".to_string(),"zmq_ipc".to_string(), true);
}
*/

#[test]
fn test_squad() {
    basic_test(TestType::SQUAD,"basic".to_string());
} 

#[test]
fn test_multi_label() {
    basic_test(TestType::MULTI,"basic".to_string());
}

#[test]
fn test_multi_match() {
    basic_test(TestType::MULTI, "python_match".to_string());
}

#[test]
fn test_single_class() {
    basic_test(TestType::SINGLE,"basic".to_string());
}

#[test]
fn test_single_match() {
    basic_test(TestType::SINGLE, "python_match".to_string());
}