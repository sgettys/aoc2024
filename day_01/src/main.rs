mod chat_gpt;
mod locations;
fn main() {
    let file_path = "data.txt";
    locations::run(file_path);
    chat_gpt::run(file_path);
}
