use regex::Regex;

// Combination of ChatGPT and EddieGPT
pub fn parse_and_compute(input: &str) {
    // Define regex patterns for `mul`, `do`, and `don't` instructions
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    // do_regex will match from "don't() to do()" non greedily
    let do_regex = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    // dont_regex will match from the last "don't()" to the end of the string
    let dont_regex = Regex::new(r"don't\(\).*?$").unwrap();
    let text = input.replace("\n", "");
    let res = do_regex.replace_all(&text, "");
    let done = dont_regex.replace_all(&res, "");
    let total_sum_of_products = mul_regex
        .captures_iter(&done)
        .map(|cap| {
            let x = cap[1].parse::<i32>().unwrap();
            let y = cap[2].parse::<i32>().unwrap();
            x * y
        })
        .sum::<i32>();
    println!("Total: {}", total_sum_of_products);
}
