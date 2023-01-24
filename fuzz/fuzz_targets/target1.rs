#![no_main]

use std::collections::BTreeMap;
use libfuzzer_sys::fuzz_target;

#[derive(Clone, Debug, arbitrary::Arbitrary)]
pub enum Op {
    Insert(u32),
    Remove(u32),
    Query(u32),
}

fn fuzz(ops: Vec<Op>) {
    let mut m: im::OrdMap<u32, u8> = im::OrdMap::default();
    let mut m2: BTreeMap<u32, u8> = BTreeMap::default();

    for op in ops {
        dbg!(&op);
        match op {
            Op::Insert(key) => {
                m.insert(key, 0);
                m2.insert(key, 0);
            },
            Op::Remove(key) => {
                m.remove(&key);
                m2.remove(&key);
            },
            Op::Query(key) => {
                let k1 = m.get_prev(&key);
                let k2 = m.range(..=key).rev().next();
                let k3 = m2.range(..=key).rev().next();
                assert_eq!(k1, k3);
                assert_eq!(k2, k3);
            },
        }
    }
}

fuzz_target!(|ops: Vec<Op>| {
    fuzz(ops);
});
