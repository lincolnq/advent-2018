use std::collections::HashSet;

pub fn advent1(s: String) -> Result<i32, &'static str> {
    let lines = s.split("\n");
    let ints = lines.flat_map(|s| i32::from_str_radix(s, 10).into_iter()).collect::<Vec<i32>>();

    let mut seen_freqs = HashSet::new();
    let mut current_freq = 0;
    for limit in 1..1000 {
        for i in ints.iter() {
            if seen_freqs.contains(&current_freq) {
                return Ok(current_freq)
            }
            seen_freqs.insert(current_freq);
            current_freq += i;
        }
    }
    Err("reached limit of attempts")
}