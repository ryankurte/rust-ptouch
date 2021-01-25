

use log::{debug};
use structopt::StructOpt;
use simplelog::{TermLogger, LevelFilter, TerminalMode};

use ptouch::{PTouch, Filter};

#[derive(Clone, Debug, PartialEq, StructOpt)]
pub struct Options {

    #[structopt(flatten)]
    filter: Filter,

    #[structopt(long, default_value = "info")]
    log_level: LevelFilter,
}

fn main() -> Result<(), anyhow::Error> {
    // Parse CLI options
    let opts = Options::from_args();

    // Setup logging
    TermLogger::init(opts.log_level, simplelog::Config::default(), TerminalMode::Mixed).unwrap();

    debug!("Connecting to PTouch device: {:?}", opts.filter);

    // Create PTouch connection
    let _ptouch = match PTouch::new(&opts.filter) {
        Ok(d) => d,
        Err(e) => {
            return Err(anyhow::anyhow!("Error connecting to PTouch: {:?}", e));
        }
    };

    debug!("Device connected!");

    // TODO: do things with the printer...

    // TODO: close the printer?

    Ok(())
}

