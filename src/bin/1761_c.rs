// https://codeforces.com/contest/1761/problem/c
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

use std::collections::HashSet;
use std::io::BufWriter;
use std::io::Write;
use std::str;

fn run(out: &mut BufWriter<impl Write>, src: &str) {
    input! {
        src = src,
        t: usize,
        tcs: [[chars;];t]
    }

    for vv in tcs {
        let n = vv.len();
        let mut sets: Vec<_> = (0..n)
            .map(|i| {
                let mut s = HashSet::new();
                s.insert(i + 1);
                s
            })
            .collect();

        for i in 0..n {
            for j in 0..n {
                if i != j &&  vv[i][j] == '1' {
                    let clone = sets[i].clone();
                    sets[j].extend(clone);
                }
            }
        }

        for s in sets {
            write!(out, "{} ", s.len()).unwrap();
            let result = s.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(" ");
            writeln!(out, "{}", result).unwrap();
        }
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
        "2
4
0001
1001
0001
0000
3
011
001
000",
    );

    let vec = out.into_inner().unwrap();
    let result = str::from_utf8(&vec).unwrap();
    assert_eq!(
        result.trim(),
        "3 1 2 3
2 1 3
2 2 4
4 1 2 3 4
1 1
2 1 2
3 1 2 3"
    );
}

