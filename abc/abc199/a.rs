// https://atcoder.jp/contests/abc199/submissions/47525531

use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32
    }

    print!("{}", if a * a + b * b < c * c {"Yes"} else {"No"});
}

