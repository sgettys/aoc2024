mod chat_gpt;
mod locations;
fn main() {
    let input = include_str!("../input/input.txt");
    locations::run(input);
    chat_gpt::run(input);
}
