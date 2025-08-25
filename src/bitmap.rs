//! Bitmap helper for PTouch raster encoding
// Rust PTouch Driver / Utility
//
// https://github.com/ryankurte/rust-ptouch
// Copyright 2021 Ryan Kurte

use crate::device::DeviceResolution;

/// Bitmap helper for encoding raster data.
/// This internally manages offsets and byte ordering for printing.
pub struct Bitmap {
    offset: usize,
    width: usize,
    resolution: DeviceResolution,
    data: Vec<Vec<u8>>,
}

impl Bitmap {
    /// Create a new bitmap object with the provided raster line offset, width, and resolution
    pub fn new(offset: usize, width: usize, resolution: DeviceResolution) -> Self {
        Self {
            offset,
            width,
            resolution,
            data: vec![]
        }
    }

    /// Add a raster line
    pub fn raster_line(&mut self, line: &[bool]) {
        let line_bytes = self.resolution.line_bytes();
        let mut e = vec![0u8; line_bytes];

        if line.len() > self.width as usize {
            panic!("Line width exceeds renderable width");
        }

        for i in 0..line.len() {
            // Skip unset pixels
            if !line[i] {
                continue;
            }

            let offset_index = self.offset + i;

            // Set pixels, note the reverse bit-order within the byte
            e[offset_index / 8] |= 1 << (7 - (offset_index % 8));
        }

        self.data.push(e);
    }

    // Fetch encoded lines for printing
    pub fn data(&self) -> Vec<Vec<u8>> {
        self.data.clone()
    }
}

