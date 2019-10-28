// `Structure`という構造体のための`fmt::Debug`をderiveする
// `Structure`は単一のi32をメンバに持っている
#[derive(Debug)]
struct Structure(i32);

// `Deep`という構造体の中に`Structure`を入れる
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // {}と{:?}は似ている
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor="actor's");

    // Structureはプリント可能
    println!("Now {:?} will print!", Structure(3));

    // `derive`による出力は表示結果をコントロールできない
    // 出力を7だけにするにはどうすればよいか？
    println!("Now {:?} will print!", Deep(Structure(7)));
}
