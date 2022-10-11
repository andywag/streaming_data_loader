use loader::masking;
use serde_yaml::Value;

#[tokio::test]
async fn test_top() {
    let f = std::fs::File::open("tests/masking_tests.yaml").unwrap();
    let config_file:Value = serde_yaml::from_reader(f).unwrap();

    //let config = MaskingConfig{batch_size:8, sequence_length:128, mask_length:18, tokenizer_name:"bert-base-uncased".to_string()};
    let _result = masking::masking_top::run_main(&config_file["basic"]).await;
}