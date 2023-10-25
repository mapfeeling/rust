
const fn rec(x: i32, n: i32) -> bool {
    if x < n { false } else if x == n { true } else {
        let mut base = 1;
        loop {
            base *= 10;
            let (x, n) = (x / base, n - x % base);
            if n < 0 { break false; }
            if rec(x, n) { break true; }
        }
    }
}
const fn make_table<const N: usize>() -> [i32; N] {
    let (mut i, mut ret) = (1, [0; N]);
    while i < N {
        let i2 = (i * i) as i32;
        ret[i] = ret[i - 1];
        if rec(i2, i as i32) { ret[i] += i2; }
        i += 1;
    }
    ret
}
const N: usize = 1000;
const TABLE: [i32; N + 1] = make_table::<{N+1}>();

impl Solution {
    pub fn punishment_number(n: i32) -> i32 {
        TABLE[n as usize]
    }
}

