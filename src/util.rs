use log::debug;
use simplelog::{LevelFilter, TermLogger, TerminalMode};
use structopt::StructOpt;

use ptouch::{
    render::{Op, Render, RenderConfig},
    Filter, PTouch,
};

#[derive(Clone, Debug, PartialEq, StructOpt)]
pub struct Options {
    #[structopt(flatten)]
    filter: Filter,

    #[structopt(subcommand)]
    command: Command,

    #[structopt(long, default_value = "info")]
    log_level: LevelFilter,
}

#[derive(Clone, Debug, PartialEq, StructOpt)]
pub enum Command {
    // Fetch printer info
    Info,

    // Render a print preview
    Preview,
}

fn main() -> anyhow::Result<()> {
    // Parse CLI options
    let opts = Options::from_args();

    // Setup logging
    TermLogger::init(
        opts.log_level,
        simplelog::Config::default(),
        TerminalMode::Mixed,
    )
    .unwrap();

    // Run commands that do not _require_ the printer
    match &opts.command {
        Command::Preview => {
            let cfg = RenderConfig::default();
            let ops = vec![Op::pad(16), Op::text("Hello\nWorld"), Op::pad(16)];
            let mut r = Render::new(cfg);

            r.render(&ops)?;

            r.show()?;

            return Ok(());
        }
        _ => (),
    }

    // Create PTouch connection
    debug!("Connecting to PTouch device: {:?}", opts.filter);

    let mut ptouch = match PTouch::new(&opts.filter) {
        Ok(d) => d,
        Err(e) => {
            return Err(anyhow::anyhow!("Error connecting to PTouch: {:?}", e));
        }
    };

    debug!("Device connected!");

    // TODO: do things with the printer...

    match &opts.command {
        Command::Info => {
            let i = ptouch.info()?;
            println!("Info: {:?}", i);
        }
        _ => (),
    }

    // TODO: close the printer?

    Ok(())
}
