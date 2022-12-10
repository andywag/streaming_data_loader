
use loader::{config::{TrainingConfig, TaskType}, tasks::{cases::BasicCases, python::python_cases}};

#[tokio::main]

async fn test_case(config:TrainingConfig, task:TaskType) {
    loader::logger::create_logger();
    let result = loader::tasks::run(config, task, Some("../../../storage".to_string())).await;
    log::info!("Result {}", result);
    assert!(result);
   
}


#[test]
fn test_bert() {
    test_case(BasicCases::Bert.get_config(true), TaskType::Mlm);
} 
#[test]
fn test_roberta() {
    test_case(BasicCases::Roberta.get_config(true), TaskType::Mlm);
} 
/* 
#[test]
fn test_gpt() {
    test_case(BasicCases::Gpt.get_config());
} 
*/
#[test]
fn test_t5() {
    test_case(BasicCases::T5.get_config(true), TaskType::Span);
} 

/* 
#[test]
fn test_squad() {
    test_case(BasicCases::Squad.get_config(true));
} 
*/


#[test]
fn test_multi_label() {
    test_case(BasicCases::Multi.get_config(true), TaskType::MultiLabel);
}


#[test]
fn test_single_class() {
    test_case(BasicCases::Single.get_config(true), TaskType::SingleClass);
}

#[test]
fn test_python() {
    test_case(BasicCases::Python.get_config(true), TaskType::Python);
}

#[test]
fn test_t5_python() {
    test_case(loader::tasks::python::python_cases::get_case(python_cases::Cases::Span, true), TaskType::Python);
}


