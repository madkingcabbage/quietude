#![allow(unused)]

use anyhow::Result;
use clap::{ArgAction, Parser};
use log::{debug, error, info, trace, warn, LevelFilter};
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
use quietude::{app::App, ui::errors::install_hooks};

#[derive(Parser)]
#[clap(name="Q", about = "Pythagorean window", author, version, long_about = None)]
struct Args {
    #[clap(long, short = 's', action=ArgAction::Set, help = "Set seed for world generation")]
    seed: Option<u32>,
    #[clap(long, short = 'l', action=ArgAction::Set, help = "Load save")]
    savename: Option<String>,
}


fn main() -> Result<()> {
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("q.log")
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

    App::new(args.seed, args.savename)?.run()?;

    Ok(())
}
