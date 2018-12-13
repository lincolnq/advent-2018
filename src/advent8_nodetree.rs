use nom::multispace0;
use parsing::*;

#[derive(Debug,PartialEq)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<i32>,
}

named!(int32_sp<&str, i32>, terminated!(int32, many1!(complete!(one_of!(" \n")))));

named!(node<&str, Node>,
    do_parse!(
        nchild: int32_sp >>
        nmeta: int32_sp >>
        children: count!(node, nchild as usize) >>
        metadata: count!(int32_sp, nmeta as usize) >>
        (Node { children: children, metadata: metadata })
    )
);

fn sum_metadata(n: &Node) -> i32 {
    return n.metadata.iter().sum::<i32>() + n.children.iter().map(sum_metadata).sum::<i32>();
}
fn value(n: &Node) -> i32 {
    match n.children.len() {
        0 => n.metadata.iter().sum::<i32>(),
        _ => {
            let child_values = n.children.iter().map(value).collect::<Vec<i32>>();
            n.metadata.iter().map(|ix| {
                let cv_ix = (ix - 1) as usize;
                match child_values.get(cv_ix) {
                    None => 0,
                    Some(v) => *v
                }
            }).sum::<i32>()
        }
    }
}

pub fn advent8(s: String) -> Result<i32, &'static str> {
    let n = node(&s).expect("Unable to parse").1;
    println!("Parsed! {:?}", n);
    Ok(value(&n))
}
