use lib::util::file_to_vec;

fn main() {
    let i = file_to_vec("input/2.txt".to_owned()).unwrap();
    let v: Vec<Vec<_>> = i
        .iter()
        .map(|x| {
            x.split(" ")
                .into_iter()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let res = v
        .iter()
        .filter(|x| x.windows(2).into_iter().fold((true, x[0] > x[1]), f).0)
        .count();

    let res2 = v
        .iter()
        .filter(|x| {
            let v = x
                .iter()
                .enumerate()
                .map(|x| (x.0, x.1.to_owned()))
                .collect::<Vec<_>>()
                .windows(2)
                .fold((x[0] > x[1], vec![]), f2)
                .1;

            v.is_empty()
                || v.iter().any(|y| {
                    let mut c = x.to_vec();
                    c.remove(*y);
                    c.windows(2).into_iter().fold((true, c[0] > c[1]), f).0
                })
        })
        .count();

    println!("res 1: {res:#?}");
    println!("res 2: {res2:#?}");
}

fn mc(x: i32, y: i32) -> (bool, bool) {
    let r = (x - y).abs();
    (x > y, 0 < r && 4 > r)
}

fn f(c: (bool, bool), i: &[i32]) -> (bool, bool) {
    let r = mc(i[0], i[1]);
    (c.0 && r.0 == c.1 && r.1, c.1)
}

fn f2((f, mut c): (bool, Vec<usize>), i: &[(usize, i32)]) -> (bool, Vec<usize>) {
    let (r, t) = mc(i[0].1, i[1].1);
    if !(t && r == f) {
        c.extend([0, i[0].0, i[1].0]);
    }
    (f, c)
}
