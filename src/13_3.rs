// コピー不可な型
// `clone()`メソッドを用いないかぎり、値のコピーではなくムーブがおきる
struct Empty;
struct Null;

// ジェネリック型`T`に対するトレイト
trait DoubleDrop<T> {
    // `self`に加えてもうひとつジェネリック型を受け取り、
    // 何もしないメソッドのシグネチャを定義
    fn double_drop(self, _: T);
}

// `U`を`self`として、`T`をもう一つの引数として受け取る`DoubleDrop<T>`を実装する。
// `U`、`T`はいずれもジェネリック型
impl<T, U> DoubleDrop<T> for U {
    // このメソッドは２つの引数の所有権を取り、メモリ上から開放する。
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null = Null;

    // `empty`と`null`を開放
    empty.double_drop(null);

    // empty;
    // null;
}
