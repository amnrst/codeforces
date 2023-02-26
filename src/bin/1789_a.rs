// https://codeforces.com/contest/1789/problem/a
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
    ($iter:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($iter, $t);
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

fn gcd(a: u32, b: u32) -> u32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    return a;
}

fn run(out: &mut BufWriter<impl Write>, src: &str) {
    input! {
        src = src,
        t: usize,
        tcs:[[u32;];t]
    }
    for tc in &tcs {
        let mut min_gcd = 101u32;
        for (i, a) in tc.iter().enumerate() {
            for b in tc.iter().skip(i + 1) {
                min_gcd = std::cmp::min(min_gcd, gcd(*a, *b));
            }
        }
        let result = if min_gcd <= 2 { "Yes" } else { "No" };
        writeln!(out, "{}", result).unwrap();

        // tc.sort();
        // dbg!(&tc);
        // let mut current_gcd = gcd(tc[0], tc[1]);
        // let mut result: String = "Yes".to_owned();
        // for (i, v) in tc.iter().enumerate() {
        //     if i >= 1 {
        //         dbg!((&current_gcd, v, &i));
        //         current_gcd = gcd(*v, current_gcd);
        //         if current_gcd > (i + 1) as u32 {
        //             result = "No".to_owned();
        //             break;
        //         }
        //     }
        // }
        // writeln!(out, "{}", result).unwrap();
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
        "6
2
3 6
3
1 2 4
3
3 6 1
3
15 35 21
4
35 10 35 14
5
1261 227821 143 4171 1941",
    );

    let vec = out.into_inner().unwrap();
    let result = str::from_utf8(&vec).unwrap();
    assert_eq!(
        result.trim(),
        "No
Yes
Yes
No
Yes
Yes"
    );
}

