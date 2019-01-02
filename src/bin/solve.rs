use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::BTreeMap,
    fs::File,
    io::{self, Read, Seek, Write},
    path::Path,
};

use fwop::{generate_map, output, parse};

const MASK: u32 = (1 << 25) - 1;

fn generate_fwopcache<P: AsRef<Path>>(path: P) -> io::Result<File> {
    println!("No game cache found, generating...");
    let mut file = File::create(path)?;
    let mut cache = BTreeMap::<u32, (u32, u32)>::new();
    for i in 0..1 << 25 {
        let map = generate_map(i);
        cache
            .entry(map)
            .and_modify(|(x, count)| {
                let new_count = i.count_ones();
                if new_count < *count {
                    *x = i;
                    *count = new_count;
                }
            })
            .or_insert_with(|| (i, i.count_ones()));
    }

    for (k, v) in &cache {
        file.write(&k.to_ne_bytes())?;
        file.write(&v.0.to_ne_bytes())?;
    }
    file.flush()?;
    Ok(file)
}

fn get_entry_from_cache(cache: &mut File, x: u32) -> Option<u32> {
    let mut base = 0;
    let mut size = 1 << 23; // Num entries in cache
                            // println!("target: {}", x);
    while size > 0 {
        size /= 2;
        let mid = base + size;
        // get this entry from cache
        let mut buf = [0; 4];
        cache.seek(io::SeekFrom::Start(mid as u64 * 8)).unwrap();
        cache
            .read_exact(&mut buf)
            .map_err(|_| {
                io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Failed to find a solution in the cache",
                )
            })
            .unwrap();

        let key = u32::from_ne_bytes(buf);
        cache
            .seek(io::SeekFrom::Start((mid as u64 * 8) + 4))
            .unwrap();
        cache
            .read_exact(&mut buf)
            .map_err(|_| {
                io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Failed to find a solution in the cache",
                )
            })
            .unwrap();

        let value = u32::from_ne_bytes(buf);

        // Do the binary search comparision
        base = match key.cmp(&x) {
            Less => mid,
            Greater => base,
            Equal => return Some(value),
        }
    }
    // check cache[start] and cache[end] fit the target and return those keys values
    return None;
}

fn solve_from_cache(cache: &mut File, x: u32) -> io::Result<()> {
    cache.seek(io::SeekFrom::Start(x as u64 * 4))?;
    let value = get_entry_from_cache(cache, x);
    match value {
        Some(y) => {
            println!("{}", output(y));
            Ok(())
        }
        None => {
            println!("Error getting value from cache.");
            Ok(())
        }
    }
}

fn main() -> io::Result<()> {
    let input = std::env::args()
        .nth(1)
        .ok_or(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Missing argument",
        ))
        .and_then(|s| parse(&s))?;

    static FNAME: &str = ".fwopcache";
    {
        File::open(FNAME).or_else(|_| generate_fwopcache(FNAME))?;
    }
    let mut cache = File::open(FNAME)?;
    solve_from_cache(&mut cache, !input & MASK)
}
