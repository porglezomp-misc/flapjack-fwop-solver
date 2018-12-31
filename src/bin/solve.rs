#![feature(duration_float)]
use std::{
    collections::BTreeMap,
    fs::File,
    io::{self, Read, Seek, Write},
    path::Path,
    time::Instant,
};

use fwop;

const MASK: u32 = (1 << 25) - 1;

fn generate_fwopcache<P: AsRef<Path>>(path: P) -> io::Result<File> {
    println!("No game cache found, generating...");
    let mut file = File::create(path)?;
    let mut cache = BTreeMap::<u32, (u32, u32)>::new();
    let start = Instant::now();
    for i in 0..1 << 25 {
        let map = fwop::generate_map(i);
        let (map, rot, refl) = fwop::canonicalize(map);
        let i = fwop::apply_transform(i, rot, refl);
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
    let generate_end = Instant::now();
    for i in 0..1 << 25 {
        let value = cache.get(&i).map(|&x| x.0).unwrap_or(0);
        file.write(&value.to_ne_bytes())?;
    }
    let write_end = Instant::now();
    println!(
        "Saved {} cache entries in {:.02} seconds.",
        cache.len(),
        (write_end - start).as_float_secs()
    );
    println!(
        "Generated: {:6.02} seconds",
        generate_end.duration_since(start).as_float_secs()
    );
    println!(
        "    Wrote: {:6.02} seconds",
        write_end.duration_since(generate_end).as_float_secs()
    );
    file.flush()?;
    Ok(file)
}

fn solve_from_cache(cache: &mut File, x: u32) -> io::Result<u32> {
    cache.seek(io::SeekFrom::Start(x as u64 * 4))?;
    let mut buf = [0; 4];
    cache.read_exact(&mut buf).map_err(|_| {
        io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Failed to find a solution in the cache",
        )
    })?;
    Ok(u32::from_ne_bytes(buf))
}

fn main() -> io::Result<()> {
    let map = std::env::args()
        .nth(1)
        .ok_or(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Missing argument",
        ))
        .and_then(|s| fwop::parse(&s))?;

    let (map, rot, refl) = fwop::canonicalize(!map & MASK);
    static FNAME: &str = ".fwopcache";
    {
        File::open(FNAME).or_else(|_| generate_fwopcache(FNAME))?;
    }
    let mut cache = File::open(FNAME)?;
    let solution = solve_from_cache(&mut cache, map)?;
    println!(
        "{}",
        fwop::output(fwop::apply_transform_inverse(solution, rot, refl))
    );
    Ok(())
}
