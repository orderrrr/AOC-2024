use std::collections::HashMap;

use lib::util::file_to_vec;

pub fn main() {
    let i = file_to_vec("input/1.txt".to_owned()).unwrap();

    let (mut f, mut s): (Vec<_>, Vec<_>) = i
        .iter()
        .map(|x| {
            let (a, b) = x.split_once("   ").unwrap();
            (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
        })
        .unzip();

    f.sort_unstable();
    s.sort_unstable();

    let mut map: HashMap<i32, i32> = HashMap::new();
    s.iter().for_each(|s| *map.entry(*s).or_insert(0) += 1);

    let res: i32 = f.iter().zip(&s).map(|(x, y)| (x - y).abs()).sum::<i32>();
    let res2 = f.iter().map(|x| x * map.get(x).unwrap_or(&0)).sum::<i32>();

    println!("res 1: {res:#?}");
    println!("res 2: {res2:#?}");
}
