use std::collections::HashMap;

pub fn primes_up_to(limit: u32) -> Vec<u32> {
    let mut composites = HashMap::new();

    (2..(limit + 1))
        .filter(|&n| if !composites.contains_key(&n) {
            composites.entry(n * n).or_insert(Vec::new()).push(n);
            true
        } else {
            for c in composites.get(&n).unwrap().to_owned() {
                composites.entry(n + c).or_insert(Vec::new()).push(c);
            }
            composites.remove(&n);
            false
        })
        .collect()
}
