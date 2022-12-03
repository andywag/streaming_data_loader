
use clap::{Parser, ValueEnum};
use loader::{tasks::cases::BasicCases};

 
#[derive(ValueEnum, Clone, Debug)]

enum Task {
    Mlm,
    Clm,
    T5,
    Squad,
    Single,
    Multi,
    Python
}



#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(long, value_enum, default_value_t=Task::Mlm)]
    task: Task,

   #[arg(long, default_value=None)]
   cache: Option<String>,

}


#[tokio::main]
async fn main()  {
    loader::logger::create_logger();

    let args = Args::parse();

    let config = match args.task {
        Task::Mlm => BasicCases::Bert,
        Task::Clm => BasicCases::Gpt,
        Task::T5 => BasicCases::T5,
        Task::Squad => BasicCases::Squad,
        Task::Single => BasicCases::Single,
        Task::Multi => BasicCases::Multi,
        Task::Python => BasicCases::Python
    };
    

    let example = config.get_config(false);

    
    let result = loader::tasks::run(example, args.cache).await;
    log::info!("Final Result {}", result);
    /* 

    let real_path = args.path;
    let real_config = args.config; 
    // Load the Config File
    let file_path = std::fs::File::open(real_path.clone());
    let f = match file_path {
        Ok(x) => x,
        Err(_) => {
            log::error!("Can't Find File {:?}", real_path);
            std::process::exit(1);
        }
    };
    let config_file_opt = serde_yaml::from_reader(f);
    let config_file:Value = match config_file_opt {
        Ok(x) => x,
        Err(_) => {
            log::error!("Error in Config File {:?}", real_path);
            std::process::exit(1);
        }
    };
    let config_value_opt = config_file.get(real_config.clone());
    let config_ptr = match config_value_opt {
        Some(x) => Arc::new(x.to_owned()),
        None => {
            log::error!("Can't Find Config File {:?}", real_config);
            std::process::exit(1);
        }
    };

    let training_config = serde_yaml::from_value::<TrainingConfig>(config_value_opt.unwrap().clone()).unwrap();
    log::info!("Training Config {:?}", training_config);

    //let config_ptr = Arc::new(config_file.get(real_config).unwrap().to_owned());
    // Run the Loader
    let result = loader::tasks::run(training_config, args.cache).await;
    log::info!("Final Result {}", result);
    */


}