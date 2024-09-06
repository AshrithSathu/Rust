fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}
fn main() {
    println!("Enter your input here:");
    let input = read_input();
    println!("You entered: {}", input);
}
