use std::sync::Arc;

use serde_yaml::Value;

enum TestType {
    MASK,
    SQUAD,
    MULTI,
    SINGLE
}

#[tokio::main]

async fn basic_test(test_type:TestType, config:String) {
    loader::create_logger();
    
    let path = match test_type {
        TestType::MASK =>  "tests/masking.yaml",
        TestType::SQUAD => "tests/squad.yaml",
        TestType::MULTI => "tests/multi_label.yaml",
        TestType::SINGLE => "tests/single_class.yaml",
    };
    
    let f = std::fs::File::open(path).unwrap();
    let config_file:Value = serde_yaml::from_reader(f).unwrap();
    let config_ptr = Arc::new(config_file.get(config).unwrap().to_owned());
    let result = loader::tasks::run(config_ptr["model"].as_str(), config_ptr.clone()).await;


    log::info!("Result {}", result);
    assert!(result);
   
}


#[test]
fn test_masking() {
    basic_test(TestType::MASK,"basic".to_string());
} 

#[test]
fn test_masking_stream() {
    basic_test(TestType::MASK,"basic_stream".to_string());
} 


#[test]
fn test_squad() {
    basic_test(TestType::SQUAD,"basic".to_string());
} 

#[test]
fn test_multi_label() {
    basic_test(TestType::MULTI,"basic".to_string());
}

//#[test]
//fn test_multi_match() {
//    basic_test(TestType::MULTI, "python_match".to_string());
//}

#[test]
fn test_single_class() {
    basic_test(TestType::SINGLE,"basic".to_string());
}

//#[test]
//fn test_single_match() {
//    basic_test(TestType::SINGLE, "python_match".to_string());
//}