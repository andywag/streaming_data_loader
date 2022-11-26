use std::sync::Arc;

use clap::Parser;
use serde_yaml::Value;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long, default_value="tests/masking.yaml")]
   path: String,
   /// Number of times to greet
   #[arg(short, long, default_value="basic")]
   config: String,

   #[arg(short, long, default_value=None)]
   cache: Option<String>,

}

/* 
#[derive(clap::Parser, Debug, Clone)]
struct Args2 {
   #[command(subcommand)]
   action: Action,
}

#[derive(clap::Subcommand, Debug, Clone)]
enum Action {
   Run {config:Option<String>, path:Option<String>},
   Download {cache:String, download:PileDatasetType, _config:Option<String>},
}
*/



#[tokio::main]
async fn main()  {
    loader::create_logger();

    let args = Args::parse();

    let real_path = args.path;//path.unwrap_or("tests/masking.yaml".to_string());
    let real_config = args.config; //config.unwrap_or("zmq_none".to_string());
    // Load the Config File
    log::info!("Path {:?}", real_path);
    let f = std::fs::File::open(real_path).unwrap();
    let config_file:Value = serde_yaml::from_reader(f).unwrap();
    let config_ptr = Arc::new(config_file.get(real_config).unwrap().to_owned());
    // Run the Loader
    let result = loader::tasks::run(config_ptr["model"].as_str(), config_ptr.clone(), args.cache).await;
    log::info!("Final Result {}", result);

    /* 
    match args.action {
        Action::Run { path, config } => {
            // Get Default Arguments
            let real_path = path.unwrap_or("tests/masking.yaml".to_string());
            let real_config = config.unwrap_or("zmq_none".to_string());
            // Load the Config File
            log::info!("Path {:?}", real_path);
            let f = std::fs::File::open(real_path).unwrap();
            let config_file:Value = serde_yaml::from_reader(f).unwrap();
            let config_ptr = Arc::new(config_file.get(real_config).unwrap().to_owned());
            // Run the Loader
            let result = loader::tasks::run(config_ptr["model"].as_str(), config_ptr.clone()).await;
            log::info!("Final Result {}", result);
        },
        Action::Download { cache, download, _config } => {
            let datasets = get_datasets(download);
            match datasets {
                Some(x) => {
                    let _ = general_file_provider::save_data_sets(cache, x).await;
                },
                None => {
                    log::error!("Data Set Not Available");
                },
            }
        }
    }
    */

}