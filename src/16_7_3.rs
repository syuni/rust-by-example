use std::fs::File;
use std::io::prelude::*;

type Result<T> = std::result::Result<T, String>;

// 下準備。ファイルを作成し文字列を書き込む
fn setup() -> std::io::Result<()> {
    let mut a = File::create("a")?;
    a.write_all(b"grape")?;

    let mut b = File::create("b")?;
    b.write_all(b"fruit")
}

// それぞれのファイルから`Result`でラップされたデータを取得
fn get_data(path: &str) -> Result<String> {
    // `try`は値を庵ラップするかエラーを返す
    let mut file = File::open(path)
        // エラーを文字列に変換する必要がある場合は今までと同様
        .map_err(|err| err.to_string())?;

    let mut contents = String::new();

    // データを`contents`に読み込む
    file.read_to_string(&mut contents)
        .map_err(|err| err.to_string())?;

    Ok(contents)
}

// ２つのファイルの内容を結合させて新しい`Result`を作成。
fn concat(a: &str, b: &str) -> Result<String> {
    let (data_a, data_b) = (get_data(a)?, get_data(b)?);

    Ok(data_a + &data_b)
}

fn main() {
    // ここの返り値のResultは無視する
    setup().unwrap();

    match concat("a", "b") {
        Ok(n) => println!("{}", n),
        Err(e) => println!("Error: {}", e),
    }
}
