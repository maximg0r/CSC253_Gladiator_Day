use std::collections::HashMap;
use std::collections::HashSet;

fn compute_intervals(v: Vec<u32>) -> Vec<Option<u32>> {
    let mut record: HashMap<u32, u32> = HashMap::new();
    v.iter()
        .enumerate()
        .map(|(i, &val)| {
            if record.contains_key(&val) {
                Some((i as u32) - record.get(&val).unwrap())
            } else {
                record.insert(val, i as u32);
                None
            }
        })
        .collect()
}

fn compute_distances(v: Vec<u32>) -> Vec<Option<u32>> {
    let mut record: HashMap<u32, HashSet<u32>> = HashMap::new();
    v.iter() // O(n)
        .map(|&val| {
            // for each entry in the map, add val to the set in that entry
            record.iter_mut().for_each(|(_, hs)| {
                // O(m)
                hs.insert(val);
            });
            if record.contains_key(&val) {
                let res = Some(record.get(&val).unwrap().len() as u32);
                record.get_mut(&val).unwrap().clear();
                res
            } else {
                let hs = HashSet::new();
                record.insert(val, hs);
                None
            }
        })
        .collect()
}

fn main() {
    // let v1 = vec![1..1000];
    // v1.append(vec![1000..1]);
    // let res = compute_distances(v1);
    // println!("{:?}", res);

    let v2 = vec![1, 2, 3, 3, 2, 1, 1];
    let res = compute_distances(v2);
    println!("{:?}", res);
}
