use nom::multispace1;
use std::collections::BTreeSet;
use itertools::Itertools;

named!(in_row<&str, (&str, &str)>,
    do_parse!(
        tag!("Step ") >>
        l: take!(1) >>
        tag!(" must be finished before step ") >>
        r: take!(1) >>
        take_until!("\n") >>
        (l, r)
    )
);

named!(in_rows<&str, Vec<(&str, &str)>>,
    separated_list_complete!(multispace1, in_row)
);

pub fn advent7(s: String) -> Result<String, &'static str> {
    let mut rows = in_rows(&s).expect("unable to parse input").1;
    println!("rows: {:?}", rows);

    let mut all_items = BTreeSet::new();
    for (l, r) in rows.iter() {
        all_items.insert(*l);
        all_items.insert(*r);
    }
    let mut completed_items = BTreeSet::new();
    let mut order = Vec::new();

    while (completed_items.len() < all_items.len()) {
        // complete the first item in all_items by looking for a rule in rows which prevents us
        for try_item in all_items.iter() {
            if completed_items.contains(try_item) {
                continue;
            }
            let mut found_blocker = false;
            for (dep, it) in rows.iter() {
                if it == try_item && !completed_items.contains(dep) {
                    found_blocker = true;
                    break;
                }
            }
            if found_blocker { continue; }

            // ok, if we are here we decided we can take the try_item
            // take it and weep
            println!("take {:?}", try_item);
            completed_items.insert(try_item);
            order.push(try_item);
            break;
        }
    }

    Ok(order.iter().join(""))
}
