use std::collections::{BTreeMap, HashMap};
fn main() {
    // let mut a = vec![1, 2, 3];
    // println!("{:?}", a);

    // swap_ed(&mut a);
    // println!("{:?}", a);

    // Usairim Isani -> hash function -> 2 bytes 123456
    // Usairim Isani -> hash function -> 2 bytes 123456
    // Usairim isani -> hash function -> 2 bytes 679345

    // let mut h = BTreeMap::new();

    // h.insert(
    //     BBQ {
    //         count: String::new(),
    //     },
    //     String::from("Hello"),
    // );
    // h.insert(
    //     BBQ {
    //         count: String::from("from"),
    //     },
    //     String::from("Hello"),
    // );

    // println!("{:#?}", h);

    // println!(
    //     "{:#?}",
    //     h.get(&BBQ {
    //         count: String::from("from"),
    //     })
    // );
    let v_to_hashmap = vec![(2, "hell"), (1, "World")];

    println!("{:#?}", v_to_hashmap);

    let hash_map = v_to_hashmap
        .into_iter()
        // .map(|x| x)
        .collect::<HashMap<u32, &str>>();

    println!("{:#?}", hash_map);

    let h_to_v = hash_map
        .into_iter()
        .map(|x| (x.1, x.0))
        .collect::<Vec<(&str, u32)>>(); // [(&str,u32),(&str,u32)]

    println!("{:#?}", h_to_v);
}
#[derive(Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HasHK {
    A { count: u32 },
    B(u8, u32, u64),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct BBQ {
    count: String,
}

// fn swap_ed(inp: &mut Vec<i32>) {
//     let len = inp.len();

//     let elem = inp[0].clone();

//     let last = inp[len - 1].clone();

//     inp[0] = last;
//     inp[len - 1] = elem;
// }
