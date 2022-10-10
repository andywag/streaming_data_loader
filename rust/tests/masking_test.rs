use loader::masking::masking_config::MaskingConfig;
use loader::masking;

#[tokio::test]
async fn test_top() {
    let config = MaskingConfig{batch_size:8, sequence_length:128, mask_length:18, tokenizer_name:"bert-base-uncased".to_string()};
    let result = masking::masking_top::run_main(config).await;
}