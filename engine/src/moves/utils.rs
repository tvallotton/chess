#[inline]
pub const fn file(n: u8) -> u64 {
    0x101010101010101 << n
}

#[inline]
pub const fn rank(n: u8) -> u64 {
    0xff << (8 * n)
}

pub fn or<const D: usize>(array: [u64; D]) -> u64 {
    array
        .iter()
        .fold(0, |x, y| x | y)
}

pub fn invert(mut x: u64) -> u64 {
    x = x >> 32 | (x << 32);

    let mask = 0x0000ffff0000ffff;
    x = (x >> 16) & mask | (x & mask) << 16;

    let mask = 0x00ff00ff00ff00ff;
    x = (x >> 8) & mask | (x & mask) << 8;

    let mask = 0x0f0f0f0f0f0f0f0f;
    x = (x >> 4) & mask | (x & mask) << 4;

    let mask = 0x3333333333333333;
    x = (x >> 2) & mask | (x & mask) << 2;

    let mask = 0x5555555555555555;
    x = (x >> 1) & mask | (x & mask) << 1;

    x
}

pub fn debug(x: u64) {
    use std::fmt::Write;
    let mut out = String::new();
    for i in 0..8 {
        for j in 0..8 {
            write!(out, "{}", (x >> (i * 8)) >> j & 1).unwrap();
        }
        writeln!(out).unwrap();
    }
    writeln!(out).unwrap();
    #[cfg(not(target_arch = "wasm32"))]
    println!("{}", out);
    #[cfg(target_arch = "wasm32")]
    log::debug!("{}", out);
}
