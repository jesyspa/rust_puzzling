use rust_puzzling::io::readpair;

fn main() {
    let mut s = String::new();
    while let Some((x, y)) = readpair::<i64>(&mut s) {
        println!("{}", (x - y).abs());
    }
}
