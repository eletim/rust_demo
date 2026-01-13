// この構造体はデータの所有者であり、String を所有する
struct Owner {
    data: String,
    count: usize,
}

// 不変借用: 所有権を移さずに読み取りだけ行う
fn show(owner: &Owner) {
    println!("show -> data: {}, count: {}", owner.data, owner.count);
}

// 可変借用: 所有権を移さずに内容を変更する
fn update(owner: &mut Owner) {
    owner.data.push_str(" + 追加");
    owner.count += 1;
    println!("update -> data: {}, count: {}", owner.data, owner.count);
}

fn main() {
    // 変数が構造体を所有し、その中の String も所有される
    let mut owner = Owner {
        data: String::from("初期データ"),
        count: 0,
    };

    // 不変借用で読み取り
    show(&owner);

    // 可変借用で変更
    update(&mut owner);

    // 変更後も同じ所有者がデータを持ち続ける
    show(&owner);
}
