use loader::masking;
use serde_yaml::Value;

#[tokio::main]
async fn basic_test(config:String) {
    let f = std::fs::File::open("tests/masking_tests.yaml").unwrap();
    let config_file:Value = serde_yaml::from_reader(f).unwrap();

    masking::masking_top::run_main(&config_file[config]).await;
}

#[tokio::test]
async fn test_top() {
    let f = std::fs::File::open("tests/masking_tests.yaml").unwrap();
    let config_file:Value = serde_yaml::from_reader(f).unwrap();

    let result = masking::masking_top::run_main(&config_file["basic"]).await;
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

/* 
#[tokio::test]
async fn test_python_ipc() {
    let f = std::fs::File::open("tests/masking_tests.yaml").unwrap();
    let config_file:Value = serde_yaml::from_reader(f).unwrap();
    let _result = masking::masking_top::run_main(&config_file["zmq_ipc"]).await;
}
*/