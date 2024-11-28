use anyhow::Result;
use clap::{ArgAction, Parser};
use log::LevelFilter;
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
    Config,
};
use quark::{app::App, ui::errors::install_hooks};

#[derive(Parser)]
#[clap(name="Q", about = "Pythagorean window", author, version, long_about = None)]
struct Args {
    #[clap(long, short = 'd', action=ArgAction::Set, help = "Project directory")]
    project_directory: Option<String>,
}


fn main() -> Result<()> {
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("quark.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(log::LevelFilter::Info)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    let args = Args::parse();

    install_hooks()?;

    log4rs::init_config(config)?;

    App::new(args.project_directory).run()?;

    Ok(())
}
