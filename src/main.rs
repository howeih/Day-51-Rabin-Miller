fn prime(n: u32) -> bool {
    let base = vec![2u64, 7u64, 61u64];
    if n == 0 || n == 1 {
        return false;
    }
    if base.iter().any(|x| { *x == n as u64 }) {
        return true;
    }
    let nm1 = n - 1;
    let mut d = nm1;
    let mut s = 0;
    while d & 1 == 0 {
        d >>= 1;
        s += 1;
    }
    let n64 = n as u64;
    for a in base {
        let mut x = 1u64;
        let mut p = a;
        let mut dr = d;
        while dr > 0 {
            if dr & 1 != 0 {
                x = x * p % n64;
            }
            p = p * p % n64;
            dr >>= 1;
        }
        if x == 1 || x as u32 == nm1 {
            continue;
        }
        let mut r = 1;
        loop {
            if r >= s {
                return false;
            }
            x = x * x % n64;
            if x == 1 {
                return false;
            }
            if x as u32 == nm1 {
                break;
            }
            r += 1;
        }
    }
    true
}

fn main() {
    for i in 0..1000 {
        let is = prime(i);
        if is {
            print!("{} ", i);
        }
    }
    print!("\n");
}
