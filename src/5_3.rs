// `NanoSecond` を `u64` の別名として使用する。
type NanoSecond = u64;
type Inch = u64;

// 警告を抑えるアトリビュート
#[allow(non_camel_case_types)]
type u64_t = u64;

// こちらも警告を抑えるアトリビュート
#[allow(trivial_numeric_casts)]
fn main() {
    // `NanoSecond` = `Inch` = `u64_t)
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    // 型のエイリアスは、元の型をより型安全にしてくれるわけではない。
    // なぜならば、エイリアスは新たに型を定義しているわけではないから。
    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );

    // エイリアスをつける一番の理由はタイプ量を減らすこと。
    // IoResult<T>型はResult<T, IoError>の別名。
}
