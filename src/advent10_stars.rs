use parsing::*;
use nom::{multispace0, multispace1};
use itertools::Itertools;

#[derive(Debug,PartialEq)]
struct Star {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

named!(in_row<&str, Star>,
    do_parse!(
        tag!("position=<") >>
        multispace0 >>
        x: int32 >>
        tag!(",") >>
        multispace0 >>
        y: int32 >>
        tag!("> velocity=<") >>
        multispace0 >>
        vx: int32 >>
        tag!(",") >>
        multispace0 >>
        vy: int32 >>
        tag!(">") >>
        (Star { x: x, y: y, vx: vx, vy: vy })
    )
);

named!(in_rows<&str, Vec<Star>>,
    separated_list_complete!(multispace1, in_row)
);

fn intersect(s1: &Star, s2: &Star) -> Option<f64> {
    // returns the approximate time that s1 and s2 will intersect
    // returns nil if no good estimate
    // (we get this by averaging the ts at which the xcoords match and ycoords match)
    // x1 + tvx1 = x2 + tvx2
    // tvx1 - tvx2 = x2 - x1
    // t(vx1 - vx2) = x2 - x1
    // t = (x2 - x1)/(vx1 - vx2)

    let mut sum_t = 0.0;
    let mut count = 0;
    let vdelta_x = s1.vx - s2.vx;
    if vdelta_x != 0 {
        sum_t += (s2.x - s1.x) as f64 / vdelta_x as f64;
        count += 1;
    }
    let vdelta_y = s1.vy - s2.vy;
    if vdelta_y != 0 {
        sum_t += (s2.y - s1.y) as f64 / vdelta_y as f64;
        count += 1;
    }

    if count > 0 {
        Some(sum_t / count as f64)
    } else {
        None
    }
}

fn advance_stars(stars: &Vec<Star>, steps: i32) -> Vec<Star> {
    stars.iter().map(|s|
        Star {
            x: s.x + (s.vx * steps),
            y: s.y + (s.vy * steps),
            vx: s.vx, vy: s.vy
        }
    ).collect()
}

fn print_stars(stars: &Vec<Star>) {
    let (minx, maxx) = stars.iter().map(|s| s.x).minmax().into_option().unwrap();
    let (miny, maxy) = stars.iter().map(|s| s.y).minmax().into_option().unwrap();

    for row in miny..=maxy {
        let mut outrow = String::new();
        for col in minx..=maxx {
            if stars.iter().find(|s| (s.x, s.y) == (col, row)).is_some() {
                outrow.push('#');
            } else {
                outrow.push('.');
            }
        }
        println!("{}", outrow);
    }
}

pub fn advent10(s: String) -> Result<String, &'static str> {
    let stars = in_rows(&s).expect("Unable to parse").1;
    println!("Stars: {:?}", stars);

    // let's figure out at ~what t the lines intersect
    let mut sum_t = 0.0;
    let mut count = 0;
    for mut sub_it in &stars.iter().chunks(2) {
        let c1o = sub_it.next();
        let c2o = sub_it.next();
        if let (Some(c1), Some(c2)) = (c1o, c2o) {
            if let (Some(int)) = intersect(c1, c2) {
                println!("c1: {:?} c2: {:?}, int: {:?}", c1, c2, int);
                sum_t += int;
                count += 1;
            }
        }
    }

    let avg_t = sum_t / (count as f64);
    println!("avg t: {}", avg_t);

    let stars2 = advance_stars(&stars, avg_t.round() as i32);
    print_stars(&stars2);

    Err("nyi")
}
