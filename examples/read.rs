use inhex::inhex;

fn main() {
    let data: Vec<u8> = inhex!("
        0fde
    ");

    let pos: usize = std::env::args().nth(1)
        .expect("expected 1 arg")
        .parse().expect("1 arg should be number");

    let bt = data[pos];

    println!("byte #{}: {}", pos, bt);
}
