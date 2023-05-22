mod util;

use util::crypt_lib::substitute_frequency;
use util::crypt_analysis::{frequency_analysis, substitute_numbers_with_letters};

use util::crypt_lib::swap_character_in_string;

fn main() {

    // get frequency
    let frequency = frequency_analysis();
    println!("Frequency Analysis :: {:?}", frequency);

    // try subsitution based on analysis of english lang
    let substitution = substitute_frequency(frequency.unwrap().clone());
    println!("Character Analysis :: {:?}", substitution);

    // my file
    let file_two = substitute_numbers_with_letters(substitution.unwrap().clone()).unwrap();
    println!("File Map :: {:?}", file_two);

    // hacky string modification
    // swap_character_in_string : swap all occurences of 'u' with 'm'
    let mod_one = swap_character_in_string(file_two.clone(), 'u', 'm');
    let mod_two = swap_character_in_string(mod_one.unwrap(), 's', 'i');
    let mod_three = swap_character_in_string(mod_two.unwrap(), 'd', 'h');
    let mod_four = swap_character_in_string(mod_three.unwrap(), 'r', 'l'); // l <-> c
    let mod_five = swap_character_in_string(mod_four.unwrap(), 'r', 'c'); // c <-> u
    let mod_six = swap_character_in_string(mod_five.unwrap(), 'r', 'u');
    let mod_seven = swap_character_in_string(mod_six.unwrap(), 'y', 'f');
    let mod_eight = swap_character_in_string(mod_seven.unwrap(), 'd', 'r');
    let mod_nine = swap_character_in_string(mod_eight.unwrap(), 'g', 'v'); // v <-> q
    let mod_ten = swap_character_in_string(mod_nine.unwrap(), 'g', 'q');
    let mod_eleven = swap_character_in_string(mod_ten.unwrap(), 'p', 'g');
    let mod_twelve = swap_character_in_string(mod_eleven.unwrap(), 'w', 'p');
    let mod_thirteen = swap_character_in_string(mod_twelve.unwrap(), 'x', 'z');
    let mod_fourteen = swap_character_in_string(mod_thirteen.unwrap(), 'j', 'x');
    let mod_fifteen = swap_character_in_string(mod_fourteen.unwrap(), 'k', 'w');

    let message = mod_fifteen.unwrap();
    println!("File Mod :: {:?}", message);

}
