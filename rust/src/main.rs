

use loader::masking::masking_config::{MaskingConfig};
use loader::masking::{self, masked_data::MaskedData};
use tokio::task;


#[tokio::main] 
async fn main()  {
    let config = MaskingConfig{batch_size:8, sequence_length:128, mask_length:18, tokenizer_name:"bert-base-uncased".to_string()};
    let result = masking::masking_top::run_main(config).await;
    std::process::exit(0);
    /* 
    let (tx, rx) = tokio::sync::mpsc::channel::<String>(2);
    let (tx_trans, rx_trans) = tokio::sync::mpsc::channel::<MaskedData>(100);

    //utils::get_tokenizer("bert-base-uncased".to_string());
    //let base = "https://dumps.wikimedia.org/other/cirrussearch/current/";
    //let location = "/home/andy/Downloads/enwiki-20220926-cirrussearch-content.json.gz".to_string();
    let config = MaskingConfig{batch_size:8, sequence_length:128, mask_length:18, tokenizer_name:"bert-base-uncased".to_string()};
    let config2 = config.clone();
    let location = "../data/test.json.gz".to_string();

    let join_rx = task::spawn(async move {
        let result = masking::masking_top::receiver(&config, rx_trans);
        result.await;
    });

    let join_tokenizer = task::spawn(async move {
        let tok = masking::masking_top::create_tokenizer(&config2, rx, tx_trans);
        tok.await;
    });

    let join_provider = task::spawn(async move {
        let base = masking::masking_top::provider(location, false, tx);
        base.await;
    });

    //let total = tokio::join!(join_rx, join_tokenizer, join_provider);

    tokio::select! {        
        _ = join_provider => {}
        _ = join_tokenizer => {}
        _ = join_rx => {
            println!("Inside Rx Loop");
            std::process::exit(0);
        }
    };
    */
}