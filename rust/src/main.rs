
use clap::{Parser, ValueEnum};
use loader::{tasks::cases::BasicCases};

#[derive(ValueEnum, Clone, Debug)]

enum Mode {
    Run,
    Filter
}
 

#[derive(ValueEnum, Clone, Debug)]

enum Task {
    Mlm,
    Clm,
    T5,
    Squad,
    Single,
    Multi,
    Python,
    PythonContext
}



#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(long, value_enum, default_value_t=Task::Mlm)]
    task: Task,
    
    #[clap(long, value_enum, default_value_t=Mode::Run)]
    mode: Mode,

    #[arg(long, default_value=None)]
    cache: Option<String>,

    #[arg(short, long, action)]
    test: bool,

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
        Task::Python => BasicCases::Python,
        Task::PythonContext => BasicCases::PythonContext

    };
    
    match args.mode {
        Mode::Run => {
            let example = config.get_config(args.test);
            let result = loader::tasks::run(example, args.cache).await;
            log::info!("Final Result {}", result);
        },
        Mode::Filter => {

        }
    }
    
   


}