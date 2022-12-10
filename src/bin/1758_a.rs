// https://codeforces.com/contest/1758/problem/a
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
        ts: [String; t]
    }
    for string in ts {
        let mut result = string.clone();
        result.push_str(&string.chars().rev().collect::<String>());
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
    run(&mut out, "4asururuerrorgornanutforajaroftuna");

    let vec = out.into_inner().unwrap();
    let result = str::from_utf8(&vec).unwrap();
    assert_eq!(
        result.trim(),
        "aa
suurruurruus
rgnororerrerorongr
aannuuttffoorraajjaarrooffttuunnaa"
    );
}
