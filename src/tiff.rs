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
    }
}
