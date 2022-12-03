use log::{LevelFilter};
use std::io::Write;

pub fn create_logger() {
    
    let _ = env_logger::Builder::new()
    .format(|buf, record| {
        writeln!(
            buf,
            "{}:{} [{}] - {}",
            record.file().unwrap_or("unknown"),
            record.line().unwrap_or(0),
            //chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
            record.level(),
            record.args()
        )
    })
    .filter(None, LevelFilter::Info)
    .try_init();

}