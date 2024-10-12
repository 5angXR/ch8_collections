#[allow(warnings)]
pub fn get_mid(list: &Vec<i32>) -> i32 {
    let num = list.len();
    if num < 1 {
        return 0;
    }
    let mut sorted_list = list.clone();
    sorted_list.sort();

    if num % 2 == 1 {
        sorted_list[num / 2]
    } else {
        sorted_list[num / 2 - 1]
    }
}
#[allow(warnings)]
pub fn get_mod(list: &Vec<i32>) -> i32 {
    let num = list.len();
    if num < 1 {
        return 0;
    }
    let mut sorted_list = list.clone();
    sorted_list.sort();

    let mut iter = list.chunk_by(|a, b| a == b);
    let mut max: usize = 0;
    let mut mod_num = 0;
    loop {
        match iter.next() {
            None => break,
            Some(same_list) => if max < same_list.len() {
                max = same_list.len();
                mod_num = same_list[0];
            }
        }
    }
    mod_num
}

pub fn calc_median(list: &[i32]) -> Option<f32> {
    let num = list.len();
    if num < 1 {
        return None;
    }
    let mut sorted_list = list.to_owned();
    sorted_list.sort();

    let is_even = num % 2 == 0;
    let mid = num / 2;

    if is_even {
        match (sorted_list.get(mid - 1), sorted_list.get(mid)) {
            (Some(&num1), Some(&num2)) => Some((num1 + num2) as f32 / 2f32),
            _ => None
        }
    } else {
        match sorted_list.get(mid) {
            Some(&b) => Some(b as f32),
            _ => None
        }
    }
}

pub fn calc_mode(list: &[i32]) -> Option<i32> {
    use std::collections::HashMap;
    let num = list.len();
    if num < 1 {
        return None;
    }

    let mut map = HashMap::new();
    for &item in list {
        map.entry(item).and_modify(|cnt| *cnt += 1).or_insert(1);
    }

    let (mut val, mut max_cnt) = (0, 0);
    for (k, cnt) in map {
        if cnt > max_cnt {
            val = k;
            max_cnt = cnt;
        }
    }

    if max_cnt < 2 {
        None
    } else {
        Some(val)
    }
}