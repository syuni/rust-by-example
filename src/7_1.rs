fn main() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");

        // この式は`i32`を返す
        10 * n
    } else {
        println!(", and is a big number, reduce by two");

        // ここでも返り値の型は`i32`でなくてはならない
        // セミコロンをつけるとUnitを返すのでエラーになる。
        n / 2
    };
    // ここのセミコロンを忘れずに

    println!("{} -> {}", n, big_n);
}
