use std::io;

use fwop::{blit, output, parse};

fn main() -> io::Result<()> {
    let mut map = std::env::args()
        .nth(1)
        .ok_or(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Missing argument",
        ))
        .and_then(|s| parse(&s))?;
    let b = std::env::args()
        .nth(2)
        .ok_or(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Missing argument",
        ))
        .and_then(|s| parse(&s))?;

    for bit in 0..25 {
        if b & 1 << bit != 0 {
            map = blit(map, bit);
        }
    }
    println!("{}", output(map));
    Ok(())
}
