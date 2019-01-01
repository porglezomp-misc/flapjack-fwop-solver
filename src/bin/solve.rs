use std::{
    collections::BTreeMap,
    fs::File,
    io::{self, Read, Seek, Write},
    path::Path,
};

use fwop::{generate_map, output, parse};

const MASK: u32 = (1 << 25) - 1;

// fn generate_fwopcache<P: AsRef<Path>>(path: P) -> io::Result<File> {
//     println!("No game cache found, generating...");
//     let mut file = File::create(path)?;
//     let mut cache = BTreeMap::<u32, (u32, u32)>::new();
//     for i in 0..1 << 25 {
//         let map = generate_map(i);
//         cache
//             .entry(map)
//             .and_modify(|(x, count)| {
//                 let new_count = i.count_ones();
//                 if new_count < *count {
//                     *x = i;
//                     *count = new_count;
//                 }
//             })
//             .or_insert_with(|| (i, i.count_ones()));
//     }
//     for i in 0..1 << 25 {
//         let value = cache.get(&i).map(|&x| x.0).unwrap_or(0);
//         file.write(&value.to_ne_bytes())?;
//     }
//     file.flush()?;
//     Ok(file)
// }

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
    for (k, v) in cache.entries() {
        file.write(k.to_ne_bytes())?;
        file.write(&v.to_ne_bytes())?;
    }
    file.flush()?;
    Ok(file)
}


fn solve_from_cache(cache: &mut File, x: u32) -> io::Result<()> {
    cache.seek(io::SeekFrom::Start(x as u64 * 4))?;
    let mut buf = [0; 4];
    cache.read_exact(&mut buf).map_err(|_| {
        io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Failed to find a solution in the cache",
        )
    })?;
    println!("{}", output(u32::from_ne_bytes(buf)));
    Ok(())
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
