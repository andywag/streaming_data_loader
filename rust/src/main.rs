


use std::sync::Arc;

use loader::provider::provider_config;
use loader::datasets::masking;
use loader::datasets::squad;

use clap::Parser;
use serde_yaml::Value;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long, default_value="tests/squad_tests.yaml")]
   path: String,

   /// Number of times to greet
   #[arg(short, long, default_value="basic_hugging")]
   config: String,
}



#[tokio::main] 
async fn main()  {

    //provider::arrow_provider::download_huggingface_dataset("xed_en_fi",Some("en_annotated"), "train");
    //provider::arrow_provider::download_huggingface_dataset("squad", None, "train");

    //let location = "/home/andy/.cache/huggingface/datasets/squad/plain_text/1.0.0/d6ec3ceb99ca480ce37cdd35555d6cb2511d223b9150cce08a837ef62ffea453/squad-train.arrow";

    let f = std::fs::File::open("tests/multi_label.yaml").unwrap();
    let config_file:Value = serde_yaml::from_reader(f).unwrap();
    let basic = config_file["basic"]["source"].to_owned();
    let result = provider_config::Source::create(basic);
    //let temp:Source = serde_yaml::from_value(basic).unwrap();
    print!("Temp {:?}", result);
    
    /* 
    let args = Args::parse();
    //println!("Args {:?}", args);
    let f = std::fs::File::open(args.path).unwrap();
    let config_file:Value = serde_yaml::from_reader(f).unwrap();
    let config_ptr = Arc::new(config_file.get(args.config).unwrap().to_owned());

    if false {
        let _result = masking::masking_top::run_main(config_ptr).await;
        println!("Final Result {}", _result);
    }
    else {
        let result = squad::squad_top::run_main(config_ptr).await;
        println!("Squad Result {}", result);
    }
    */
    
}