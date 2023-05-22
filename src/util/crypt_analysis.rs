use indexmap::map::IndexMap;

use super::file_handling::read_file_handler;
use super::crypt_lib::sort_by_occurence;

pub fn frequency_analysis() -> std::io::Result<IndexMap<i32, i32>> {

    let result = read_file_handler()?;

    // Turn the result into a vector by splitting the string by spaces, then collect into a 
    // hashmap with the count of each characters and transform into a vector of numbers
    let result = result.split(" ").collect::<Vec<&str>>();

    // convert each item in the vector to a number from a string
    let converted_result = result.iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let sorted_result = sort_by_occurence(converted_result);

    // Note: Hashtable does not gurantee order of insertion
    // Hashtable -> does not gurantee order of insertion (we don't want)
    // BTreeMap -> orders by key at default (we don't want)
    // IndexMap -> orders by insertion (we want)
    let mut frequency: IndexMap<i32, i32> = IndexMap::new();
    // let mut frequency = std::collections::HashMap::new();
    // let mut frequency = std::collections::BTreeMap::new();

    // iterate over the result adding the count of each character to the frequency hashtable
    for i in sorted_result {

        let count = frequency.entry(i).or_insert(0);
        *count += 1;
    }

    Ok(frequency)
}

// replace numbers in a given file with values from an indexmap 
pub fn substitute_numbers_with_letters(map: IndexMap<i32, &'static str>) -> std::io::Result<String> {

    let result = read_file_handler()?;
    let result = result.split(" ").collect::<Vec<&str>>();
    let result = result.iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut new_result = String::new();

    for i in result.iter() {
        if let Some(new_value) = map.get(i) {
            new_result.push_str(new_value);
            new_result.push_str(" ");
        }
    }

    Ok(new_result)
}
