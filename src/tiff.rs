//! TIFF compression functions
// Rust PTouch Driver / Utility
//
// https://github.com/ryankurte/rust-ptouch
// Copyright 2021 Ryan Kurte

#[derive(Clone, Debug, PartialEq)]
enum CompressMode {
    None(u8),
    Repeated(u8, usize),
    Unique(Vec<u8>),
}

// TODO: incomplete implementation, does not consider > 16 case from docs
pub fn compress(data: &[u8]) -> Vec<u8> {
    let mut c = Vec::<u8>::new();

    let mut state = CompressMode::None(data[0]);

    // Perform byte-wise compression
    for i in 1..data.len() {
        state = match state {
            CompressMode::None(v) if data[i] == v => CompressMode::Repeated(v, 1),
            CompressMode::None(v) => CompressMode::Unique(vec![v, data[i]]),
            CompressMode::Repeated(v, n) if data[i] == v => CompressMode::Repeated(v, n + 1),
            CompressMode::Repeated(v, n) => {
                let count = 0xFF - (n as u8 - 1);

                c.push(count as u8);
                c.push(v);

                CompressMode::None(data[i])
            }
            CompressMode::Unique(mut v) if data[i] != v[v.len() - 1] => {
                v.push(data[i]);

                CompressMode::Unique(v)
            }
            CompressMode::Unique(v) => {
                let count = v.len() - 1;

                c.push(count as u8);
                c.extend_from_slice(&v[..count]);

                CompressMode::Repeated(data[i], 2)
            }
        };
    }

    // Finalize any pending data
    match state {
        CompressMode::None(v) => {
            c.push(0x00);
            c.push(v);
        }
        CompressMode::Repeated(v, n) => {
            let count = 0xFF - (n as u8 - 1);

            c.push(count as u8);
            c.push(v);
        }
        CompressMode::Unique(v) => {
            let count = v.len() - 1;

            c.push(count as u8);
            c.extend_from_slice(&v);
        }
    }

    // If the encoded length > 16, just use this in simple mode.
    if c.len() > 16 {
        c = vec![];
        c.push(data.len() as u8);
        c.extend_from_slice(data);
    }

    c
}

pub fn uncompress(data: &[u8]) -> Vec<u8> {
    let mut u = vec![];
    let mut i: usize = 0;

    loop {
        let d = data[i] as i8;

        if d < 0 {
            // -ve indicates repeated chars
            let mut r = vec![data[i+1]; (-d+1) as usize];
            u.append(&mut r);
            i += 2;
        } else {
            // +ve indicates literal sequence
            let c = d as usize;
            u.extend_from_slice(&data[i+1..i+c+2]);
            i += c + 2;
        }

        if i >= data.len() {
            break;
        }
    }

    return u
}

#[cfg(test)]
mod test {
    #[test]
    fn test_raster_compression() {
        let uncompressed = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x22, 0x22, 0x23, 0xBA, 0xBF, 0xA2, 0x22, 0x2B,
        ];
        let compressed = [
            0xED, 0x00, 0xFF, 0x22, 0x05, 0x23, 0xBA, 0xBF, 0xA2, 0x22, 0x2B,
        ];

        let c = super::compress(&uncompressed);

        assert_eq!(
            c, compressed,
            "Compressed: {:02x?} Expected: {:02x?}",
            &c, &compressed
        );

        let d = super::uncompress(&compressed);

        assert_eq!(
            d, uncompressed,
            "Uncompressed: {:02x?} Expected: {:02x?}",
            &d, &uncompressed
        );
    }

    // TODO: test compress / decompress as something is definitely not -right-
}
