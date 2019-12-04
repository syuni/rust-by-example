fn main() {
    use std::mem;

    let color = "green";

    // `color`をプリントするためのクロージャ。
    // これは`color`を借用（`&`）し、その借用とクロージャを`print`という名の変数に保持する。
    // 借用は`print`がスコープから出るまで続く。
    // `println!`は参照を与えれば機能するので、これ以上なにかする必要はない。
    let print = || println!("`color`: {}", color);

    // 借用を行ったクロージャをコールする。
    print();
    print();

    let mut count = 0;

    // `count`をインクリメントするためのクロージャ。
    // `count`と`&mut count`の両方を取ることができるが、後者のほうが制約が少ないため、
    // （`count`だと`&mut count`と違い、一度しか呼ぶことができない。）
    // そちらを取る。直後の`count`を借用する。

    // `int`には`mut`をつける必要がある。なぜならミュータブルな型が内部で使用されているからである。
    // ミュータブルなクロージャは呼ぶたびに内部変数を変更する。
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // クロージャを実行
    inc();
    inc();

    // let reborrow = &mut count;

    // 代入によるコピーではなくムーブが起きる型
    // （ヒープ上の値へのポインタであるため、コピーすると所有者が２つできてしまう。）
    let movable = Box::new(3);

    // `mem::drop`は`T`（ジェネリック型）を取る必要があるため、このクロージャは
    // 参照ではなく値を取る。その場合、もしもコピー可能な値ならば、
    // 元の値はそのままでコピーのみを取る。不可能ならば値そのものを移動させる。
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume`は変数を消費（開放）するため、一度しか呼び出すことができない。
    consume();
    // consume();
}
