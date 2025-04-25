use solana_block_parser_test::myrc::my_rc::MyRc;


fn main() {
    let a = MyRc::new("hello".to_string());
    let b = a.clone();
    let c = b.clone();

    println!("Count a: {}", a.strong_count()); // 3
    drop(b);
    println!("Count after b drop: {}", a.strong_count()); // 2
    drop(c);
    println!("Count after c drop: {}", a.strong_count()); // 1
}