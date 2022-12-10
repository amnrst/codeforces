// https://codeforces.com/contest/1758/problem/c
macro_rules! input{
    ($iter:expr) => {};
    ($iter:expr,) => {};
    (src = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input!{iter, $($r)*}
    };
    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input!{$iter $($r)*}
    };
}
macro_rules! read_value {
    ($iter:expr, ($($t:tt),* )) => { ( $(read_value!($iter, $t)),* ) };
    ($iter:expr, [$t:tt;]) => { read_value!($iter, [$t; read_value!($iter, usize)]) };
    ($iter:expr, [$t:tt;$len:expr]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };
    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };
    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

use std::io::BufWriter;
use std::io::Write;
use std::str;

// assuming n % x == 0
// return the smaller number 'p' such that
// n % p == 0 and p % x == 0
fn f(n: u32, x: u32) -> u32 {
    let m = n / x;
    let mut p = 2;
    while p * p <= m {
        if m % p == 0 {
            return p * x;
        }
        p = p + 1;
    }
    return n;
}

fn run(out: &mut BufWriter<impl Write>, src: &str) {
    input! {
        src = src,
        t: usize,
        tcs:[(u32,u32); t]
    }
    for (n, mut x) in tcs {
        if n % x != 0 {
            writeln!(out, "-1").unwrap();
            continue;
        }
        write!(out, "{} ", x).unwrap();
        let mut idx = 2;
        while idx < n {
            if idx != x {
                write!(out, "{} ", idx).unwrap();
            } else {
                let p = f(n, x);
                write!(out, "{} ", p).unwrap();
                x = p
            }
            idx = idx + 1;
        }
        writeln!(out, "{}", 1).unwrap();
    }
}

fn main() {
    let out = std::io::stdout();
    let mut out = std::io::BufWriter::new(out.lock());
    let s = {
        use std::io::Read;
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s
    };
    run(&mut out, &s);
}

#[test]
fn test_0() {
    let vec = Vec::new();
    let mut out = BufWriter::with_capacity(100, vec);
    run(
        &mut out,
        "3
3 3
4 2
5 4",
    );

    let vec = out.into_inner().unwrap();
    let result = str::from_utf8(&vec).unwrap();
    assert_eq!(
        result.trim(),
        "3 2 1
2 4 3 1
-1"
    );
}

