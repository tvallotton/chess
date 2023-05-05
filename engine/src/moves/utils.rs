

#[inline]
pub const fn file(n: u8) -> u64 {
    0x101010101010101 << n
}

#[inline]
pub const fn rank(n: u8) -> u64 {
    0xff << (8 * n)
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

pub const fn invert_u8(mut x: u8) -> u8 {
    x = x >> 4 | (x << 4);

    let mask = 0x33;
    x = (x >> 2) & mask | (x & mask) << 2;

    let mask = 0x55;
    x = (x >> 1) & mask | (x & mask) << 1;

    x
}

pub fn debug(x: u64) {
    for i in 0..8 {
        for j in 0..8 {
            print!("{}", (x >> (i * 8)) >> j & 1);
        }
        println!();
    }
    println!();
}
