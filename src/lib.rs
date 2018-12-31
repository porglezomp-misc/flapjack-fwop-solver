use std::io;

fn parse_chunk(x: &str) -> Option<u32> {
    let mut res = 0;
    for c in x.chars().rev() {
        res <<= 1;
        res |= match c {
            '#' => 1,
            '-' => 0,
            _ => return None,
        }
    }
    Some(res)
}

pub fn parse(x: &str) -> io::Result<u32> {
    if x.len() != 29 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "Expected 29 characters (like ####-|---##|##-#-|#-##-|#-#-#), got {}",
                x.len()
            ),
        ));
    }
    let mut res = 0;
    for chunk in x.split('|').rev() {
        if chunk.len() != 5 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "Incorrect chunk length, expected 5 character chunk, got {}",
                    chunk.len()
                ),
            ));
        }
        res <<= 5;
        res |= parse_chunk(chunk)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid chunk"))?;
    }
    Ok(res)
}

pub fn output(mut x: u32) -> String {
    fn output_chunk(x: &mut u32, buf: &mut String) {
        for _ in 0..5 {
            match *x & 1 {
                1 => buf.push('#'),
                _ => buf.push('-'),
            };
            *x >>= 1;
        }
    }

    let mut output = String::with_capacity(29);
    output_chunk(&mut x, &mut output);
    for _ in 0..4 {
        output.push('|');
        output_chunk(&mut x, &mut output);
    }
    output
}

pub fn blit(mut map: u32, pos: u32) -> u32 {
    if pos >= 5 {
        map ^= 1 << (pos - 5);
    }
    if pos % 5 != 0 {
        map ^= 1 << (pos - 1);
    }
    map ^= 1 << pos;
    if pos % 5 != 4 {
        map ^= 1 << (pos + 1);
    }
    if pos < 20 {
        map ^= 1 << (pos + 5);
    }
    map
}

pub fn generate_map(code: u32) -> u32 {
    let mut map = 0;
    for i in 0..25 {
        if code & 1 << i != 0 {
            map = blit(map, i);
        }
    }
    map
}

#[test]
fn test_blit() {
    assert_eq!(blit(0, 00), 0b0000100011);
    assert_eq!(blit(0, 01), 0b0001000111);
    assert_eq!(blit(0, 02), 0b0010001110);
    assert_eq!(blit(0, 03), 0b0100011100);
    assert_eq!(blit(0, 04), 0b1000011000);
    assert_eq!(blit(0, 05), 0b000010001100001);
    assert_eq!(blit(0, 06), 0b000100011100010);
    assert_eq!(blit(0, 07), 0b001000111000100);
    assert_eq!(blit(0, 08), 0b010001110001000);
    assert_eq!(blit(0, 09), 0b100001100010000);
    assert_eq!(blit(0, 10), 0b00001000110000100000);
    assert_eq!(blit(0, 11), 0b00010001110001000000);
    assert_eq!(blit(0, 12), 0b00100011100010000000);
    assert_eq!(blit(0, 13), 0b01000111000100000000);
    assert_eq!(blit(0, 14), 0b10000110001000000000);
    assert_eq!(blit(0, 15), 0b0000100011000010000000000);
    assert_eq!(blit(0, 16), 0b0001000111000100000000000);
    assert_eq!(blit(0, 17), 0b0010001110001000000000000);
    assert_eq!(blit(0, 18), 0b0100011100010000000000000);
    assert_eq!(blit(0, 19), 0b1000011000100000000000000);
    assert_eq!(blit(0, 20), 0b0001100001000000000000000);
    assert_eq!(blit(0, 21), 0b0011100010000000000000000);
    assert_eq!(blit(0, 22), 0b0111000100000000000000000);
    assert_eq!(blit(0, 23), 0b1110001000000000000000000);
    assert_eq!(blit(0, 24), 0b1100010000000000000000000);
}

#[test]
fn test_generate_map() {
    // This is right by manual inspection
    assert_eq!(generate_map(0b100000001000001011), 4674152);
}

#[test]
fn test_output_parse() {
    static TEXT: &str = "##---|---#-|###-#|#-#--|###-#";
    assert_eq!(output(parse(TEXT).unwrap()), TEXT);
}
