// 请从命令行读取一行字符串（例如 "apple banana pear banana apple banana"）。
// 使用空格进行拆分，统计每个单词出现的次数，并以从高到底的顺序输出。
// 如果出现次数相同，按单词本身的字典序从小到大排序输出。

use std::collections::HashMap;
//use std::io::{self, BufRead};

/// 从命令行读取一行字符串，统计每个单词出现的次数，并以从高到底的顺序输出。
/// 如果出现次数相同，按单词本身的字典序从小到大排序输出。
pub fn question3(input: &str) -> Vec<(String, usize)> {
    let mut counts = HashMap::new();
    let words = input.split_whitespace();

    for word in words {
        *counts.entry(word.to_string()).or_insert(0) += 1;
    }

    let mut counts_vec: Vec<(String, usize)> = counts.into_iter().collect();
    counts_vec.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    counts_vec
}
