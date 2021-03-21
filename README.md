# Brother P-Touch Raster Driver (and utility)

Brother P-Touch Label-Maker Raster Driver for `PT-E550W/P750W/P710BT` devices.


## Status

***Extremely alpha, tested only on the `PT-P710BT`, API subject to change***

[![GitHub tag](https://img.shields.io/github/tag/ryankurte/rust-ptouch.svg)](https://github.com/ryankurte/rust-ptouch)
![Build Status](https://github.com/ryankurte/rust-ptouch/workflows/Rust/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/ptouch.svg)](https://crates.io/crates/ptouch)
[![Docs.rs](https://docs.rs/ptouch/badge.svg)](https://docs.rs/ptouch)


## Usage

### Utility

The utility supports a set of basic subcommands:

- `ptouch-util [SUBCOMMAND] --help` to show help options
- `ptouch-util [--media MEDIA] render --file=[OUTPUT] [OPTIONS]` to render to an `OUTPUT` image file
- `ptouch-util [--media MEDIA] preview [OPTIONS]` to render to a preview window (not available on all platforms)
- `ptouch-util print [OPTIONS]` to print

The `--media` argument sets the default media type when the printer is unavailable, otherwise this is loaded from the printer.

Each of `render`, `preview`, and `print` take a set of `[OPTIONS]` to configure the output, these options are:

- `text VALUE [--font=FONT]` to render text in the specified font, use `\n` for newlines
- `qr CODE` to render a QRCode with the provided value
- `qr-text CODE VALUE [--font=FONT]` to render a QRCode followed by text
- `image FILE` to render an image directly
- `template FILE` to load a `.toml` render template (see [example.toml](example.toml))
- `barcode CODE` to render a barcode (experimental, missing config options)

These CLI options are a subset of those available using the library intended to provide the basics. If you think there's something missing, feel free to open an issue / PR!

### API

This needs cleaning up before it's _reasonable_ to use... for usage see [src/util.rs](src/util.rs).

## Resources

- [PT-P710BT Manual](https://support.brother.com/g/b/manualtop.aspx?c=eu_ot&lang=en&prod=p710bteuk)
- [Brother Raster Command Reference](https://download.brother.com/welcome/docp100064/cv_pte550wp750wp710bt_eng_raster_101.pdf)
- [Pytouch Cube python driver](https://github.com/piksel/pytouch-cube)
