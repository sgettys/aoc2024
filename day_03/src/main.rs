mod chat_gpt;
mod corrupted;

fn main() {
    let input = include_str!("../input/data.txt");
    corrupted::parse_and_compute(input);
    chat_gpt::parse_and_compute(input);
}
