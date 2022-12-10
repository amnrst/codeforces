// https://codeforces.com/contest/1758/problem/d
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

fn run(out: &mut BufWriter<impl Write>, src: &str) {
    input! {
        src = src,
        t: usize,
        tcs: [i32; t]
    }

    for n in tcs {
        let k = n + 1;
        let mut v = vec![k; n as usize];

        if n % 2 == 1 {
            let mid = n / 2;
            for i in 0..n {
                v[i as usize] += i - mid;
            }
        } else {
            let mid = n / 2;
            for i in 0..mid {
                v[i as usize] += i - mid;
                v[(n - 1 - i) as usize] += mid - i;
            }
        }

        for e in v.iter_mut() {
            *e = *e + 1;
        }

        if n%2 == 1 {
            v[0] -= 1;
            v[(n-1) as usize] += 1;
            v[(n-2) as usize] += 1;
        } else {
            v[(n-1) as usize] += 1;
        }

        let result = v
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        writeln!(out, "{}", result).unwrap();
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
        &mut out, "5
2
3
4
5
6
",
    );

    let vec = out.into_inner().unwrap();
    let result = str::from_utf8(&vec).unwrap();
    assert_eq!(
        result.trim(),
        "3 1
20 29 18 26 28
25 21 23 31"
    );
}

