use std::collections::{HashMap, BTreeMap};
use indexmap::map::IndexMap;

// replace the occurence count of each character with the character for the most frequent in
// english language
pub fn substitute_frequency(frequency: IndexMap<i32, i32>) -> std::io::Result<IndexMap<i32, &'static str>> {

    let mut map: IndexMap<i32, &str> = IndexMap::new();
    let english_language_frequency = english_language_frequency();
    let mut english_language_frequency_iterator = english_language_frequency
        .keys()
        .cloned()
        .collect::<Vec<_>>().into_iter();

    for key in frequency.keys() {
        if let Some(new_value) = english_language_frequency_iterator.next() {
            map.insert(*key, new_value);
        }
    }

    Ok(map)
}

pub fn sort_by_occurence(mut vec: Vec<i32>) -> Vec<i32> {

    // count
    let mut occurrences: HashMap<i32, i32> = HashMap::new();
    for &num in vec.iter() {
        *occurrences.entry(num).or_insert(0) += 1;
    }

    // sort by occurrences
    vec.sort_by_key(|&num| std::cmp::Reverse(occurrences[&num]));

    vec
}

pub fn replace_character_in_string(string: String, old: char, new: char) 
    -> std::io::Result<String> {

    let mut new_string = String::new();

    for c in string.chars() {
        if c == old {
            new_string.push(new);
        } else {
            new_string.push(c);
        }
    }

    Ok(new_string)
}

pub fn swap_character_in_string(string: String, first: char, second: char) 
    -> std::io::Result<String> {

    let mut new_string = String::new();

    for c in string.chars() {
        if c == first {
            new_string.push(second);
        } else if c == second {
            new_string.push(first);
        } else {
            new_string.push(c);
        }
    }

    Ok(new_string)
}

pub fn _order_btreemap_by_value<K: Ord, V: Ord>(map: BTreeMap<K, V>) -> BTreeMap<K, V> {

    let mut sorted_map: BTreeMap<K, V> = BTreeMap::new();
    let mut entries: Vec<(K, V)> = map.into_iter().collect();
    entries.sort_by(|a, b| a.1.cmp(&b.1));

    for (key, value) in entries {
        sorted_map.insert(key, value);
    }

    sorted_map
}

// create a hashtable containing the most frequent characters in the english language
pub fn english_language_frequency() -> IndexMap <&'static str, f64> {

    // create a hashtable containing the most frequent characters in the english language
    let mut english_frequency = IndexMap::new();

    // top half
    english_frequency.insert("e", 12.7);
    english_frequency.insert("t", 9.10);
    english_frequency.insert("a", 8.2);
    english_frequency.insert("o", 7.5);
    english_frequency.insert("n", 7.0);
    english_frequency.insert("i", 6.7);
    english_frequency.insert("h", 6.3);
    english_frequency.insert("s", 6.0);
    english_frequency.insert("r", 4.0);
    english_frequency.insert("l", 4.3);
    english_frequency.insert("d", 2.8);
    english_frequency.insert("u", 2.8);
    english_frequency.insert("c", 2.4);

    // bottom half
    english_frequency.insert("m", 2.4);
    english_frequency.insert("w", 2.4);
    english_frequency.insert("y", 2.2);
    english_frequency.insert("f", 2.0);
    english_frequency.insert("g", 2.0);
    english_frequency.insert("p", 1.9);
    english_frequency.insert("b", 1.5);
    english_frequency.insert("v", 1.0);
    english_frequency.insert("k", 0.8);
    english_frequency.insert("j", 0.2);
    english_frequency.insert("x", 0.2);
    english_frequency.insert("q", 0.1);
    english_frequency.insert("z", 0.1);

    english_frequency
}
