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

type ItemSet<'a> = BTreeSet<&'a str>;
type ItemDeps<'a> = Vec<(&'a str, &'a str)>;

pub fn find_runnable<'a>(deps: &ItemDeps, all_items: &'a ItemSet, completed_items: &ItemSet) -> Option<&'a str> {
    for try_item in all_items.iter() {
        if completed_items.contains(try_item) {
            continue;
        }
        let mut found_blocker = false;
        for (dep, it) in deps.iter() {
            if it == try_item && !completed_items.contains(dep) {
                found_blocker = true;
                break;
            }
        }
        if found_blocker { continue; }
        return Some(try_item);
    }
    return None
}

pub fn advent7(s: String) -> Result<String, &'static str> {
    let mut rows = in_rows(&s).expect("unable to parse input").1;
    println!("rows: {:?}", rows);

    let mut all_items = BTreeSet::new();
    for (l, r) in rows.iter() {
        all_items.insert(*l);
        all_items.insert(*r);
    }
    let mut completed_items = BTreeSet::new();
    //let mut running_items = BTreeSet::new();
    let mut order = Vec::new();


    while (completed_items.len() < all_items.len()) {

        // complete the first item in all_items by looking for a rule in rows which prevents us
        let result = { find_runnable(&rows, &all_items, &completed_items) };

        match result {
            Some(try_item) => {
                // ok, if we are here we decided we can take the try_item
                println!("take {:?}", try_item);
                completed_items.insert(try_item);
                order.push(try_item);
                //break;
            },
            _ => return Err("Out of runnable items")
        }
    }

    Ok(order.iter().join(""))
}
