/*
#include <algorithm>
#include <array>
#include <iostream>
int main() {
    constexpr int bound = 10000;
    std::array<int, bound+1> gnumbers;
    std::array<bool, bound/2+1> excluded;
    for (int i = 0; i <= bound; ++i) {
        auto e_begin = excluded.begin();
        auto e_end = e_begin + i/2;
        std::fill(e_begin, e_end, false);
        for (int j = 1; j < (i+1)/2; ++j) {
            int const k = i - j;
            excluded[gnumbers[j] ^ gnumbers[k]] = true;
        }
        gnumbers[i] = std::find(e_begin, e_end, false) - e_begin;
    }
    for (int i = 0; i <= bound; ++i)
        std::cout << i << ' ' << gnumbers[i] << '\n';
} // Eric M. Schmidt, Jan 04 2017 */

use std::{fs::File, io::{BufWriter, Write}};

use moka::sync::Cache;

fn compute(i: usize, cache: &Cache<usize, usize>) -> usize {
    let mut excluded = vec![false; i / 2 + 1];
    for j in 1..(i + 1) / 2 {
        let k = i - j;
        unsafe {
            *excluded.get_unchecked_mut(
                cache.get(&j).unwrap_or_else(|| compute(j, cache))
                    ^ cache.get(&k).unwrap_or_else(|| compute(k, cache))
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
