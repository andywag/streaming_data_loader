


use clap::{Parser, ValueEnum};
use loader::{tasks::{cases::BasicCases, python::{python_cases, python_runner}}, config::{TaskType, ModelType}};


#[derive(ValueEnum, Clone, Debug)]

enum Mode {
    Run,
    Filter,
    Context
}
 


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    
    #[clap(long, value_enum, default_value_t=ModelType::Bert)]
    model: ModelType,

    #[clap(long, value_enum, default_value_t=TaskType::Mlm)]
    task: TaskType,
    
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
        TaskType::Mlm => BasicCases::Bert,
        TaskType::Clm => BasicCases::Gpt,
        TaskType::Span => BasicCases::T5,
        TaskType::Squad => BasicCases::Squad,
        TaskType::SingleClass => BasicCases::Single,
        TaskType::MultiLabel => BasicCases::Multi,
        TaskType::Python => BasicCases::Python,
        TaskType::Context => BasicCases::PythonContext,
        TaskType::SpanPython => BasicCases::PythonSpan,
    };
    
    match args.mode {
        Mode::Run => {
            let example = config.get_config(args.test);
            let result = loader::tasks::run(example, args.task, args.cache, None).await;
            log::info!("Final Result {}", result);
        },
        Mode::Filter => {

        },
        Mode::Context => {
            let example = python_cases::get_case(python_cases::Cases::Context, false);
            let result = python_runner::run_context(example, args.cache).await;
            log::info!("Final Result {}", result);
        }
    }
    
   


}