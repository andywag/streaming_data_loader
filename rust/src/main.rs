
use clap::{Parser, ValueEnum};
use loader::{tasks::{cases::BasicCases, python::{python_cases, python_runner}}};



#[derive(ValueEnum, Clone, Debug)]

enum Mode {
    Run,
    Filter,
    Context
}
 

#[derive(ValueEnum, Clone, Debug)]

enum Model {
    Bert,
    Roberta,
    Gpt2,
    T5,
}

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
    
    #[clap(long, value_enum, default_value_t=Model::Bert)]
    model: Model,

    #[clap(long, value_enum, default_value_t=Task::Multi)]
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

    };
    
    match args.mode {
        Mode::Run => {
            let example = config.get_config(args.test);
            let result = loader::tasks::run(example, args.cache).await;
            log::info!("Final Result {}", result);
        },
        Mode::Filter => {

        },
        Mode::Context => {
            let example = python_cases::get_case(python_cases::Cases::Context, true);
            let result = python_runner::run(example, args.cache).await;
            log::info!("Final Result {}", result);
        }
    }
    
   


}