// https://atcoder.jp/contests/abc195/submissions/47525566

use proconio::input;

fn main() {
    input! {
        m: u32,
        h: u32
    }

    print!("{}", if h % m == 0 {"Yes"} else {"No"});
}

