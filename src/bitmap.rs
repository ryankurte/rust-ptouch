
/// Bitmap helper for encoding raster data.
/// This internally manages offsets and byte ordering for printing.
pub struct Bitmap {
    offset: usize,
    width: usize,
    data: Vec<[u8; 16]>,
}

impl Bitmap {
    /// Create a new bitmap object with the provided raster line offset / width
    pub fn new(offset: usize, width: usize) -> Self {
        Self {
            offset,
            width,
            data: vec![]
        }
    }

    /// Add a raster line
    pub fn raster_line(&mut self, line: &[bool]) {
        let mut e = [0u8; 16];

        if line.len() > self.width as usize {
            panic!("Line width exceeds renderable width");
        }

        for i in 0..line.len() {
            // Skip unset pixels
            if !line[i] {
                continue;
            }

            let offset_index = self.offset + i;

            // Set pixels, not the reverse bit-order within the byte
            e[offset_index / 8] |= 1 << (7 - (offset_index % 8));
        }

        self.data.push(e);
    }

    // Fetch encoded lines for printing
    pub fn data(&self) -> Vec<[u8; 16]> {
        self.data.clone()
    }
}

