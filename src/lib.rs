#![feature(reverse_bits)]

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

fn spread(x: u32) -> u32 {
    (x & 0b10000) << 20 - 4
        | (x & 0b01000) << 15 - 3
        | (x & 0b00100) << 10 - 2
        | (x & 0b00010) << 05 - 1
        | (x & 0b00001)
}

fn rev5(x: u32) -> u32 {
    x.reverse_bits() >> (32 - 5)
}

// 01234    49EJO
// 56789    38DIN
// ABCDE -> 27CHM
// FGHIJ    16BGL
// KLMNO    05AFK
pub fn rotate(code: u32) -> u32 {
    spread(rev5(code))
        | spread(rev5(code >> 05)) << 1
        | spread(rev5(code >> 10)) << 2
        | spread(rev5(code >> 15)) << 3
        | spread(rev5(code >> 20)) << 4
}

pub fn reflect(code: u32) -> u32 {
    const MASK5: u32 = (1 << 5) - 1;
    (code & MASK5) << 20
        | (code >> 05 & MASK5) << 15
        | (code >> 10 & MASK5) << 10
        | (code >> 15 & MASK5) << 05
        | (code >> 20 & MASK5)
}

pub fn canonicalize(x: u32) -> (u32, u8, bool) {
    let mut result = (x, 0, false);
    for should_reflect in &[false, true] {
        let mut map = if *should_reflect { reflect(x) } else { x };
        for i in 0..4 {
            if map < result.0 {
                result = (map, i, *should_reflect);
            }
            map = rotate(map);
        }
    }
    result
}

pub fn apply_transform(mut x: u32, rotations: u8, should_reflect: bool) -> u32 {
    if should_reflect {
        x = reflect(x);
    }
    for _ in 0..rotations {
        x = rotate(x);
    }
    x
}

pub fn apply_transform_inverse(mut x: u32, rotations: u8, should_reflect: bool) -> u32 {
    for _ in 0..(4 - rotations) % 4 {
        x = rotate(x);
    }
    if should_reflect {
        x = reflect(x);
    }
    x
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::{arbitrary::any, bits, prop_assert, prop_assert_eq, proptest, proptest_helper};

    fn maps() -> bits::BitSetStrategy<u32> {
        bits::u32::between(0, 25)
    }

    proptest! {
        #[test]
        fn parse_doesnt_crash(ref s in r#"\PC*"#) {
            let _ = parse(s);
        }

        #[test]
        fn parse_output_inverse(ref s in r#"[#-]{5}(\|[#-]{5}){4}"#) {
            prop_assert_eq!(s, &output(parse(s).unwrap()));
        }

        #[test]
        fn output_parse_inverse(x in maps()) {
            prop_assert_eq!(x, parse(&output(x)).unwrap())
        }

        #[test]
        fn reflect_involute(x in maps()) {
            prop_assert_eq!(x, reflect(reflect(x)));
        }


        #[test]
        fn reflect_rotate3_reflect_is_rotate(x in maps()) {
            prop_assert_eq!(rotate(x), reflect(rotate(rotate(rotate(reflect(x))))));
        }

        #[test]
        fn rotate2_is_reflect_rotate2_reflect(x in maps()) {

            prop_assert_eq!(rotate(rotate(x)), reflect(rotate(rotate(reflect(x)))));
        }

        #[test]
        fn rotate3_is_reflect_rotate_reflect(x in maps()) {
            prop_assert_eq!(rotate(rotate(rotate(x))), reflect(rotate(reflect(x))));
        }

        #[test]
        fn rev5_popcnt(x in maps()) {
            const MASK5: u32 = (1 << 5) - 1;
            prop_assert_eq!(rev5(x).count_ones(), (x & MASK5).count_ones());
        }

        #[test]
        fn rev5_reion(x in maps()) {
            const MASK5: u32 = (1 << 5) - 1;
            prop_assert_eq!(rev5(x), rev5(x) & MASK5);
        }

        #[test]
        fn transform_inverses(m in maps(), rotate in 0u8..4, reflect in any::<bool>()) {
            prop_assert_eq!(
                m,
                apply_transform(apply_transform_inverse(m, rotate, reflect), rotate, reflect)
            );
            prop_assert_eq!(
                m,
                apply_transform_inverse(apply_transform(m, rotate, reflect), rotate, reflect)
            );
        }

        #[test]
        fn canonical_best(m in MAPS) {
            let (c, rot, refl) = canonicalize(m);
            prop_assert!(c <= m);
            prop_assert_eq!(c, apply_transform(m, rot, refl));
            prop_assert_eq!(m, apply_transform_inverse(c, rot, refl));
        }

        #[test]
        fn canonical_generate_commute(i in MAPS) {
            let m = generate_map(i);
            let (c, rot, refl) = canonicalize(m);
            let i2 = apply_transform(i, rot, refl);
            let c2 = generate_map(i2);
            prop_assert_eq!(c, c2);
            prop_assert_eq!(i, apply_transform_inverse(i2, rot, refl));
        }
    }

    #[test]
    fn test_blit() {
        assert_eq!(blit(0, 00), 0b_00000_00000_00000_00001_00011);
        assert_eq!(blit(0, 01), 0b_00000_00000_00000_00010_00111);
        assert_eq!(blit(0, 02), 0b_00000_00000_00000_00100_01110);
        assert_eq!(blit(0, 03), 0b_00000_00000_00000_01000_11100);
        assert_eq!(blit(0, 04), 0b_00000_00000_00000_10000_11000);
        assert_eq!(blit(0, 05), 0b_00000_00000_00001_00011_00001);
        assert_eq!(blit(0, 06), 0b_00000_00000_00010_00111_00010);
        assert_eq!(blit(0, 07), 0b_00000_00000_00100_01110_00100);
        assert_eq!(blit(0, 08), 0b_00000_00000_01000_11100_01000);
        assert_eq!(blit(0, 09), 0b_00000_00000_10000_11000_10000);
        assert_eq!(blit(0, 10), 0b_00000_00001_00011_00001_00000);
        assert_eq!(blit(0, 11), 0b_00000_00010_00111_00010_00000);
        assert_eq!(blit(0, 12), 0b_00000_00100_01110_00100_00000);
        assert_eq!(blit(0, 13), 0b_00000_01000_11100_01000_00000);
        assert_eq!(blit(0, 14), 0b_00000_10000_11000_10000_00000);
        assert_eq!(blit(0, 15), 0b_00001_00011_00001_00000_00000);
        assert_eq!(blit(0, 16), 0b_00010_00111_00010_00000_00000);
        assert_eq!(blit(0, 17), 0b_00100_01110_00100_00000_00000);
        assert_eq!(blit(0, 18), 0b_01000_11100_01000_00000_00000);
        assert_eq!(blit(0, 19), 0b_10000_11000_10000_00000_00000);
        assert_eq!(blit(0, 20), 0b_00011_00001_00000_00000_00000);
        assert_eq!(blit(0, 21), 0b_00111_00010_00000_00000_00000);
        assert_eq!(blit(0, 22), 0b_01110_00100_00000_00000_00000);
        assert_eq!(blit(0, 23), 0b_11100_01000_00000_00000_00000);
        assert_eq!(blit(0, 24), 0b_11000_10000_00000_00000_00000);
    }

    #[test]
    fn test_rotate_example() {
        // 01234    49EJO
        // 56789    38DIN
        // ABCDE -> 27CHM
        // FGHIJ    16BGL
        // KLMNO    05AFK

        // 11001    11001
        // 01111    01000
        // 00000 -> 01011
        // 00100    11000
        // 00101    10000
        assert_eq!(
            rotate(0b_00000_00000_00000_00000_00000),
            0b_00000_00000_00000_00000_00000
        );

        assert_eq!(
            rotate(0b_11111_11111_11111_11111_11111),
            0b_11111_11111_11111_11111_11111
        );
    }

    #[test]
    fn test_spread() {
        assert_eq!(spread(0b11111), 0b_00001_00001_00001_00001_00001);
    }

    #[test]
    fn test_generate_map() {
        // This is right by manual inspection
        assert_eq!(generate_map(0b_00000_00100_00000_10000_01011), 4674152);
    }

    #[test]
    fn test_output_parse() {
        static TEXT: &str = "##---|---#-|###-#|#-#--|###-#";
        assert_eq!(output(parse(TEXT).unwrap()), TEXT);
    }
}
