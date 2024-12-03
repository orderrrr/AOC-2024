use anyhow::Result;
use lib::util::file_to_string;
use regex::Regex;

fn main() -> Result<()> {
    let text = file_to_string("input/3.txt".to_string())?.replace("\n", "");
    let text_sub = Regex::new(r"(?ms)don't\(\).*?(?:do\(\)|$)")?.replace_all(&text, "");

    let res = find(&text);
    let res2 = find(&text_sub);

    println!("res: {res:#?}");
    println!("res2: {res2:#?}");

    Ok(())
}

fn find(text: &str) -> i32 {
    Regex::new(r"mul\(([\d]+),([\d]+)\)")
        .unwrap()
        .captures_iter(text)
        .map(|cap| {
            cap.get(1).unwrap().as_str().parse::<i32>().unwrap()
                * cap.get(2).unwrap().as_str().parse::<i32>().unwrap()
        })
        .sum::<i32>()
}
