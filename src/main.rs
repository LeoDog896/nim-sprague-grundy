// Eric M. Schmidt, Jan 04 2017 
use std::{fs::File, io::{BufWriter, Write}};

use quick_cache::sync::Cache;

fn compute(i: usize, cache: &Cache<usize, usize>) -> usize {
    let mut excluded = vec![false; i / 2 + 1];
    for j in 1..(i + 1) / 2 {
        let k = i - j;
        unsafe {
            *excluded.get_unchecked_mut(
                cache.get_or_insert_with(&j, || Ok::<usize, ()>(compute(j, cache))).unwrap()
                    ^ cache.get_or_insert_with(&k, || Ok::<usize, ()>(compute(k, cache))).unwrap()
            ) = true;
        }
    }

    excluded.iter().position(|&x| !x).unwrap()
}

fn main() {
    let cache: Cache<usize, usize> = Cache::new(1_000_000);

    let file = File::create("output.txt").unwrap();
    let mut writer = BufWriter::new(file);

    let bound = usize::MAX;
    for i in 0..=bound {
        let value = compute(i, &cache);
        cache.insert(i, value);
        writeln!(writer, "{}", value).unwrap();
    }
}
