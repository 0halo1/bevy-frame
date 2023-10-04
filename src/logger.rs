use chrono::Local;
use log::*;
use std::fs::File;
use std::io::Write;

pub(crate) fn logger_setup() {
    let target = Box::new(File::create("log.txt").expect("Can't create file"));
    env_logger::Builder::new()
        .target(env_logger::Target::Pipe(target))
        .filter(None, LevelFilter::Info)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
}
