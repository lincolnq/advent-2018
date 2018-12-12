use parsing::int32;
use nom::multispace1;
use itertools::Itertools;
use std::iter::Map;
use std::ops::Range;
use itertools::structs::Product;
use std::ops::RangeInclusive;
use std::collections::BTreeMap;

named!(in_row<&str, (i32, i32)>,
    do_parse!(
        x: int32 >>
        tag!(", ") >>
        y: int32 >>
        take_until!("\n") >>
        (x, y)
    )
);

named!(in_rows<&str, Vec<(i32, i32)>>,
    separated_list_complete!(multispace1, in_row)
);

fn manhattan_dist(p0: &(i32, i32), p1: &(i32, i32)) -> i32 {
    (p1.0 - p0.0).abs() + (p1.1 - p0.1).abs()
}

fn ix2p(ix: i32, origin: (i32, i32), ncols: i32) -> (i32, i32) {
    let (ox, oy) = origin;
    (ox + (ix % ncols), oy + (ix / ncols))
}


#[derive(Debug,PartialEq)]
struct Grid {
    // Construct a Grid with the extents (four corners) of the grid
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

impl Grid {
    fn size(&self) -> (i32, i32) {
        // number of columns and rows
        (1 + self.right - self.left, 1 + self.bottom - self.top)
    }
    fn ix2p(&self, ix: i32) -> (i32, i32) {
        let (ncols, nrows) = self.size();
        (self.left + (ix % ncols), self.top + (ix / ncols))
    }
    fn num_sq(&self) -> i32 {
        let size = self.size();
        size.0 * size.1
    }
    fn iter_squares(&self) -> Product<RangeInclusive<i32>, RangeInclusive<i32>> {
        iproduct!(self.left..=self.right, self.top..=self.bottom)
    }
    fn is_on_edge(&self, p: (i32, i32)) -> bool {
        let (x,y) = p;
        return x == self.left || x == self.right || y == self.top || y == self.bottom;
    }
}

#[derive(Debug,PartialEq)]
enum OwnedRegion {
    Infinite,
    Finite(Vec<(i32, i32)>)
}

pub fn advent6(s: String) -> Result<i32, &'static str> {
    let mut rows = in_rows(&s).expect("unable to parse coords").1;
    println!("rows={:?}", rows);

    let (&minx, &maxx) = rows.iter().map(|(x,y)| x).minmax().into_option().unwrap();
    let (&miny, &maxy) = rows.iter().map(|(x,y)| y).minmax().into_option().unwrap();

    let grid = Grid {left: minx, top: miny, right: maxx, bottom: maxy};

    let mut sq_in_region = 0;
    let target_dist = 10000;

    for p0 in grid.iter_squares() {
        let mut total_dist = 0;
        for p1 in rows.iter() {
            let d = manhattan_dist(&p0, p1);
            total_dist += d;
        }
        if total_dist < target_dist {
            sq_in_region += 1;
        }
    }

    Ok(sq_in_region)
}
