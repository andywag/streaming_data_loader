use std::sync::Arc;

use loader::datasets::{masking, squad, multi_label};
use serde_yaml::Value;

enum TestType {
    _MASK,
    SQUAD,
    MULTI
}

#[tokio::main]

async fn basic_test(test_type:TestType, config:String) {
    loader::create_logger();

    let path = match test_type {
        TestType::_MASK =>  "tests/masking_tests.yaml",
        TestType::SQUAD => "tests/squad_tests.yaml",
        TestType::MULTI => "tests/multi_label.yaml",
    };
    
    let f = std::fs::File::open(path).unwrap();
    let config_file:Value = serde_yaml::from_reader(f).unwrap();
    let config_ptr = Arc::new(config_file.get(config).unwrap().to_owned());

    let result = match test_type {
        TestType::_MASK =>  masking::masking_runner::run(config_ptr).await,
        TestType::SQUAD => squad::squad_runner::run(config_ptr).await,
        TestType::MULTI => multi_label::multi_runner::run(config_ptr).await,
    };
    log::info!("Result {}", result);
    assert!(result);
    /* 
    if masking {
        masking::masking_runner::run(config_ptr).await;
    }
    else {
        squad::squad_runner::run(config_ptr).await;
        //squad::squad_top::run_main(config_ptr).await;
    }
    */
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