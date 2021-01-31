

use log::{debug};
use structopt::StructOpt;
use simplelog::{TermLogger, LevelFilter, TerminalMode};

use ptouch::{PTouch, Filter};

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
}

fn main() -> anyhow::Result<()> {
    // Parse CLI options
    let opts = Options::from_args();

    // Setup logging
    TermLogger::init(opts.log_level, simplelog::Config::default(), TerminalMode::Mixed).unwrap();

    debug!("Connecting to PTouch device: {:?}", opts.filter);

    // Create PTouch connection
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
        },
    }

    // TODO: close the printer?

    Ok(())
}

