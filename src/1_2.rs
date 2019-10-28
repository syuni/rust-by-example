fn main() {
    println!("{} days", 31);

    // 引数の順序で指定
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 名前を指定
    println!("{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over");

    // フォーマット (:)
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // "     1"
    println!("{number:>width$}", number=1, width=6);

    // "000001"
    println!("{number:>0width$}", number=1, width=6);

    // チェック
    // println!("My name is {0}, {1} {0}", "Bond");
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // i32を保持するStructureという構造体
    struct Structure(i32);

    // これは動作しない
    // println!("This struct `{}` won't print...", Structure(3));

    println!("Pi is rough ly {:.*}", 3, 22.0/7.0);
}
