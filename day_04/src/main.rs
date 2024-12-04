mod word_search;
fn main() {
    let input = include_str!("../input/input.txt");
    word_search::run(input);
}
