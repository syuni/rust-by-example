struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    {
        let borrowed_point = &point;
        let another_borrowd = &point;

        // データは元々の持ち主と参照の両方からアクセスすることができます。
        println!(
            "Point has coordinates: ({}, {}, {})",
            borrowed_point.x, another_borrowd.y, point.z
        );

        // エラー！pointはすでにイミュータブルに借用されているため、
        // ミュータブルに借用することができない。
        // let mutable_borrow = &mut point;

        // ここでイミュータブルな参照がスコープを抜ける。
    }

    {
        let mutable_borrow = &mut point;

        // ミュータブルなリファレンスを介してデータを変更する
        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;

        // エラー！`point`はすでにミュータブルに借用されているため、
        // イミュータブルに借用することはできない。
        // let y = &point.y;

        // エラー！`point`はイミュータブルはリファレンスを取るため、printできません。
        // println!("Point Z coordinate is {}", point.z);

        // ここでミュータブルな参照がスコープを抜ける。
    }

    // pointへのイミュータブルな参照を使うことが再び許される。
    println!(
        "Point now has coordinates: ({}, {}, {})",
        point.x, point.y, point.z
    );
}
