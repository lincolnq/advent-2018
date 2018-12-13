use std::collections::BTreeMap;

pub fn advent2(s: String) -> Result<i32, &'static str> {
    let ids = s.split("\n").filter(|&l| l != "").collect::<Vec<&str>>();

    for (i, left) in ids.iter().enumerate() {
        for right in ids.split_at(i+1).1 {

            if left.len() != right.len() {
                continue
            }

            let matches = left.chars().zip(right.chars()).filter(|(lc, rc)| lc == rc).count();
            if matches >= left.len() - 1 {
                println!("({}, {}, {:?})", left, right, matches);
            }
        }
    }

    Err("nyi")
}
