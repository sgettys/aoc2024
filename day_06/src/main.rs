mod guard_path;
fn main() {
    let input = include_str!("../input/input.txt");
    guard_path::run(input);
}
