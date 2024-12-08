mod bridge_repair;

fn main() {
    let input = include_str!("../input/input.txt");
    bridge_repair::run(input);
}
