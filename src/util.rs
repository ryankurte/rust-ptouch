use log::debug;
use simplelog::{LevelFilter, TermLogger, TerminalMode};
use structopt::StructOpt;

use ptouch::{Filter, PTouch, device::{MediaWidth, PrintInfo}, render::{Op, Render, RenderConfig}};

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

    // Fetch printer status
    Status,

    // Render a print preview
    Preview,

    // Placeholder to preview rendering on configured tape
    Preview2,

    // Print data!
    Print,
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

            let ops = vec![
                Op::pad(16),
                Op::qr("https://hello.world"),
                Op::text("Hello World\nHow's it going?"), 
                Op::pad(16)];
            
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

    debug!("Device connected! reading status");

    let status = ptouch.status()?;
    let media = MediaWidth::from((status.media_kind, status.media_width));

    debug!("Status: {:?} media: {:?}", status, media);

    let render_cfg = RenderConfig{
        y: media.area().1 as usize,
        ..Default::default()
    };

    // TODO: do things with the printer...

    match &opts.command {
        Command::Info => {
            let i = ptouch.info()?;
            println!("Info: {:?}", i);
        },
        Command::Status => {
            println!("Status: {:?}", status);
        },
        Command::Preview2 => {
            let ops = vec![
                Op::pad(16),
                Op::qr("https://hello.world"),
                Op::text("Hello World\nHow's it going?"), 
                Op::pad(16)];
            
            let mut r = Render::new(render_cfg);

            r.render(&ops)?;

            r.show()?;
        },
        Command::Print => {
            let ops = vec![
                Op::pad(16),
                Op::qr("https://hello.world"),
                Op::text("Hello World\nHow's it going?"), 
                Op::pad(16)];
            
            let mut r = Render::new(render_cfg);

            r.render(&ops)?;

            let data = r.raster(media.area())?;

            let info = PrintInfo {
                width: Some(status.media_width),
                length: Some(0),
                raster_no: data.len() as u32,
                ..Default::default()
            };
            ptouch.print_raw(data, &info)?;

        },
        _ => (),
    }

    // TODO: close the printer?

    Ok(())
}
