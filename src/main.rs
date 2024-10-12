use ch8_collections::exercise::{num, pig_latin, txt_it};
use std::collections::HashMap;

fn main() {
    let num_list = vec![44, 121, -8, 0, 3];
    // let num_list = vec![];
    // let num_list = vec![2, 2, -9, 3, 3, -9, -9, 0];
    let mut sorted_list = num_list.clone();
    sorted_list.sort();
    println!("sorted_list is: {:?}", sorted_list);
    println!("get_mid is: {}", num::get_mid(&sorted_list));
    println!("get_mod is: {}", num::get_mod(&sorted_list));

    println!("calc_median is: {:?}", num::calc_median(&sorted_list));
    println!("calc_mode is: {:?}", num::calc_mode(&sorted_list));

    let test_str = "wuck the whole world eh";
    println!("strings_to_pig_latin is: {:?}", pig_latin::strings_to_pig_latin(test_str));
    

    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    let res = txt_it::txt_it_cmd("Add Wong to Embedded Software", &mut company);
    println!("{:?}", res);
    let res = txt_it::txt_it_cmd("Add Li to Manager", &mut company);
    println!("{:?}", res);
    let res = txt_it::txt_it_cmd("Add Tan to Embedded Software", &mut company);
    println!("{:?}", res);
    let res = txt_it::txt_it_cmd("Add Chen to Embedded Software", &mut company);
    println!("{:?}", res);
}
