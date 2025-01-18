//! Rust PTouch Command Line Utility
//
// https://github.com/ryankurte/rust-ptouch
// Copyright 2021 Ryan Kurte

use log::{debug, info, warn};
use simplelog::{LevelFilter, TermLogger, TerminalMode, ColorChoice};
use clap::{Parser, Subcommand};

use ptouch::{Options, PTouch, render::RenderTemplate};
use ptouch::device::{Media, PrintInfo, Status};
use ptouch::render::{FontKind, Op, Render, RenderConfig};


#[derive(Clone, Debug, PartialEq, Parser)]
pub struct Flags {
    #[command(flatten)]
    options: Options,

    #[command(subcommand)]
    command: Command,

    #[arg(long, default_value="16")]
    /// Padding for start and end of renders
    pad: usize,

    #[arg(value_enum, default_value="tze12mm")]
    /// Default media kind when unable to query this from printer
    media: Media,

    #[arg(long, default_value = "info")]
    log_level: LevelFilter,
}

#[derive(Clone, Debug, PartialEq, Subcommand)]
pub enum RenderCommand {
    /// Basic text rendering
    Text {
        /// Text value
        text: String,
        #[arg(value_enum, default_value="12x16")]
        /// Text font
        font: FontKind,
    },
    /// QR Code with text
    QrText {
        /// QR value
        qr: String,

        /// Text value
        text: String,

        #[arg(value_enum, default_value="12x16")]
        /// Text font
        font: FontKind,
    },
    /// QR Code
    Qr {
        /// QR value
        qr: String,
    },
    /// Datamatrix
    Datamatrix {
        /// Datamatrix value
        dm: String,
    },
    /// Barcode (EXPERIMENTAL)
    Barcode {
        /// Barcode value
        code: String,
    },
    /// Render from template
    Template{
        /// Template file
        file: String,
    },
    /// Render from image
    Image{
        /// Image file
        file: String,
    },
    /// Render example
    Example,
}

#[derive(Clone, Debug, PartialEq, Subcommand)]
pub enum Command {
    // Fetch printer info
    Info,

    // Fetch printer status
    Status,

    // Render and display a preview
    Preview{
        #[command(subcommand)]
        cmd: RenderCommand,
    },

    // Render to an image file
    Render{
        #[arg(long)]
        /// Image file to save render output
        file: String,

        #[command(subcommand)]
        cmd: RenderCommand,
    },

    // Print data!
    Print{
        #[arg(long)]
        /// Do not feed and cut label after printing to avoid waste
        chain: bool,

        #[command(subcommand)]
        cmd: RenderCommand,
    },
}

fn main() -> anyhow::Result<()> {
    // Parse CLI options
    let opts = Flags::parse();

    // Setup logging
    TermLogger::init(
        opts.log_level,
        simplelog::Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    // Create default render configuration
    let mut rc = RenderConfig{
        y: opts.media.area().1 as usize,
        ..Default::default()
    };

    debug!("Connecting to PTouch device: {:?}", opts.options);

    // Attempt to connect to ptouch device to inform configuration
    let connect = match PTouch::new(&opts.options) {
        Ok(mut pt) => {
            let status;
            if opts.options.no_status_fetch {
                info!("Connected! status request disabled, using default status...");
                // Getting default status
                status = Status::new(&opts.media)?;
                info!("Device status (default one used): {:?}", status);
            } else {
                info!("Connected! fetching status...");
                // Fetch device status
                status = pt.status()?;
                info!("Device status (fetched from device): {:?}", status);
            }

            // Build MediaWidth from status message to retrieve offsets
            let media = Media::from((status.media_kind, status.media_width));

            // Update render config to reflect tape
            rc.y = media.area().1 as usize;
            // TODO: update colours too?
            
            // Return device and mediat width
            Ok((pt, status, media))
        },
        Err(e) => Err(e),
    };


    // TODO: allow RenderConfig override from CLI


    // Run commands that do not _require_ the printer
    match &opts.command {
        #[cfg(feature = "preview")]
        Command::Preview{ cmd } => {
            // Inform user if print boundaries are unset
            if connect.is_err() {
                warn!("Using default media: {}, override with `--media` argument", opts.media);
            }

            // Load render operations from command
            let ops = cmd.load(opts.pad)?;
            
            // Create renderer
            let mut r = Render::new(rc);

            // Apply render operations
            r.render(&ops)?;

            // Display render output
            r.show()?;

            return Ok(());
        },
        #[cfg(not(feature = "preview"))]
        Command::Preview{ _cmd } => {
            warn!("Preview not enabled (or not supported on this platform");
            warn!("Try `render` command to render to image files");
            return Ok(())
        }
        Command::Render{ file, cmd } => {
            // Inform user if print boundaries are unset
            if connect.is_err() {
                warn!("Using default media: {}, override with `--media` argument", opts.media);
            }

            // Load render operations from command
            let ops = cmd.load(opts.pad)?;
            
            // Create renderer
            let mut r = Render::new(rc);

            // Apply render operations
            r.render(&ops)?;

            // Display render output
            r.save(file)?;

            return Ok(());
        },
        _ => (),
    }

    // Check PTouch connection was successful
    let (mut ptouch, status, media) = match connect {
        Ok(d) => d,
        Err(e) => {
            return Err(anyhow::anyhow!("Error connecting to PTouch: {:?}", e));
        }
    };

    // Run commands that -do- require the printer
    match &opts.command {
        Command::Info => {
            let i = ptouch.info()?;
            println!("Info: {:?}", i);
        },
        Command::Status => {
            println!("Status: {:?}", status);
        },
        Command::Print{ chain, cmd } => {
 
            // Load render operations from command
            let ops = cmd.load(opts.pad)?;
            
            // Create renderer
            let mut r = Render::new(rc);

            // Apply render operations
            r.render(&ops)?;

            // Generate raster data for printing
            let data = r.raster(media.area())?;
            
            // Setup print info based on media and rastered data
            let info = PrintInfo {
                width: Some(status.media_width),
                length: Some(0),
                raster_no: data.len() as u32,
                chain: *chain,
                ..Default::default()
            };

            // Print the thing!
            ptouch.print_raw(data, &info)?;

        },
        _ => (),
    }

    // TODO: close the printer?

    Ok(())
}


impl RenderCommand {
    pub fn load(&self, pad: usize) -> Result<Vec<Op>, anyhow::Error> {
        match self {
            RenderCommand::Text { text, font } => {
                let ops = vec![
                    Op::pad(pad),
                    Op::text_with_font(text, *font),
                    Op::pad(pad),
                ];
                Ok(ops)
            },
            RenderCommand::QrText { qr, text, font } => {
                let ops = vec![
                    Op::pad(pad),
                    Op::qr(qr),
                    Op::text_with_font(text, *font), 
                    Op::pad(pad)
                ];
                Ok(ops)
            },
            RenderCommand::Qr { qr } => {
                let ops = vec![
                    Op::pad(pad),
                    Op::qr(qr),
                    Op::pad(pad)
                ];
                Ok(ops)
            },
            RenderCommand::Datamatrix { dm } => {
                let ops = vec![
                    Op::pad(pad),
                    Op::datamatrix(dm),
                    Op::pad(pad)
                ];
                Ok(ops)
            },
            RenderCommand::Barcode { code } => {
                let ops = vec![
                    Op::pad(pad),
                    Op::barcode(code),
                    Op::pad(pad)
                ];
                Ok(ops)
            },
            RenderCommand::Template { file } => {
                // Read template file
                let t = std::fs::read_to_string(file)?;
                // Parse to render ops
                let c: RenderTemplate = toml::from_str(&t)?;
                // Return render operations
                Ok(c.ops)
            },
            RenderCommand::Image { file } => {
                let ops = vec![
                    Op::pad(pad),
                    Op::image(file),
                    Op::pad(pad)
                ];
                Ok(ops)
            }
            RenderCommand::Example => {
                let ops = vec![
                    Op::pad(pad),
                    Op::qr("https://hello.world"),
                    Op::text("hello world,,\nhow's it going?"), 
                    Op::pad(pad)
                ];

                Ok(ops)
            }
        }
    }
}

