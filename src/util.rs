

use log::{debug, warn};
use simplelog::{LevelFilter, TermLogger, TerminalMode};
use structopt::StructOpt;
use strum::VariantNames;

use ptouch::{Options, PTouch};
use ptouch::device::{Media, PrintInfo};
use ptouch::render::{FontKind, Op, Render, RenderConfig};


#[derive(Clone, Debug, PartialEq, StructOpt)]
pub struct Flags {
    #[structopt(flatten)]
    options: Options,

    #[structopt(subcommand)]
    command: Command,

    #[structopt(long, default_value="16")]
    /// Padding for start and end of renders
    pad: usize,

    #[structopt(long, default_value="70")]
    /// Default media width when unable to query this from printer
    media_width: usize,

    #[structopt(long, default_value = "info")]
    log_level: LevelFilter,
}

#[derive(Clone, Debug, PartialEq, StructOpt)]
pub enum RenderCommand {
    /// Basic text rendering
    Text {
        /// Text value
        text: String,
        #[structopt(long,  possible_values = &FontKind::VARIANTS, default_value="12x16")]
        /// Text font
        font: FontKind,
    },
    /// QR Code with text
    QrText {
        /// QR value
        qr: String,
        
        /// Text value
        text: String,

        #[structopt(long,  possible_values = &FontKind::VARIANTS, default_value="12x16")]
        /// Text font
        font: FontKind,
    },
    /// QR Code
    Qr {
        /// QR value
        qr: String,
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
    /// Render example
    Example,
}

#[derive(Clone, Debug, PartialEq, StructOpt)]
pub enum Command {
    // Fetch printer info
    Info,

    // Fetch printer status
    Status,

    // Render a print preview
    Preview(RenderCommand),

    // Print data!
    Print(RenderCommand),
}

fn main() -> anyhow::Result<()> {
    // Parse CLI options
    let opts = Flags::from_args();

    // Setup logging
    TermLogger::init(
        opts.log_level,
        simplelog::Config::default(),
        TerminalMode::Mixed,
    )
    .unwrap();

    // Create default render configuration
    let mut rc = RenderConfig{
        y: opts.media_width,
        ..Default::default()
    };

    debug!("Connecting to PTouch device: {:?}", opts.options);

    // Attempt to connect to ptouch device to inform configuration
    let connect = match PTouch::new(&opts.options) {
        Ok(mut pt) => {
            debug!("Connected! fetching status...");

            // Fetch device status
            let status = pt.status()?;
            debug!("Device status: {:?}", status);

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
    if let Command::Preview(cmd) = &opts.command {
        // Inform user if print boundaries are unset
        if connect.is_err() {
            warn!("Using default media width ({} px)", opts.media_width);
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
        Command::Print(cmd) => {
 
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
                let ops: Vec<Op> = toml::from_str(&t)?;
                // Return render operations
                Ok(ops)
            },
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
