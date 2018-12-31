use criterion::{criterion_group, criterion_main, Criterion, Fun};

fn blit_table(map: u32, pos: u32) -> u32 {
    map ^ [
        0b0000100011,
        0b0001000111,
        0b0010001110,
        0b0100011100,
        0b1000011000,
        0b000010001100001,
        0b000100011100010,
        0b001000111000100,
        0b010001110001000,
        0b100001100010000,
        0b00001000110000100000,
        0b00010001110001000000,
        0b00100011100010000000,
        0b01000111000100000000,
        0b10000110001000000000,
        0b0000100011000010000000000,
        0b0001000111000100000000000,
        0b0010001110001000000000000,
        0b0100011100010000000000000,
        0b1000011000100000000000000,
        0b0001100001000000000000000,
        0b0011100010000000000000000,
        0b0111000100000000000000000,
        0b1110001000000000000000000,
        0b1100010000000000000000000,
    ][pos as usize]
}

fn blit_cond(mut map: u32, pos: u32) -> u32 {
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

fn bench(criterion: &mut Criterion) {
    criterion.bench_functions(
        "blit",
        vec![
            Fun::new("table", |bench, map| {
                bench.iter(|| (0..25).fold(*map, blit_table))
            }),
            Fun::new("cond", |bench, map| {
                bench.iter(|| (0..25).fold(*map, blit_cond))
            }),
        ],
        42,
    );
}

criterion_group!(benches, bench);
criterion_main!(benches);
