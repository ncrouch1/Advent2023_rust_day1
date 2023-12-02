fn main() {
    let mut sum = 0;
    let num_strs = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut mappings = std::collections::HashMap::new();
    for i in 0..num_strs.len() {
        mappings.insert(num_strs[i].to_string(), (i + 1) as i32);
    }

    for line in std::fs::read_to_string("src/puzzleinput.txt").unwrap().lines() {
        let (mut first_char, mut last_char): (char, char) = (char::MAX, char::MAX);
        for c in line.chars() {
            if c.is_digit(10) { 
                first_char = c;
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_digit(10) {
                last_char = c;
                break;
            }
        }
        let mut first_char_index = i32::MAX;
        let mut last_char_index = i32::MIN;
        if !line.find(first_char).is_none() {
            first_char_index = line.find(first_char).unwrap() as i32;
        }
        if !line.rfind(last_char).is_none() { 
            last_char_index = line.rfind(last_char).unwrap() as i32;
        }
        let (mut first_str, mut first_str_index, mut last_str, mut last_str_index): (i32, i32, i32, i32) = (-1, i32::MAX, -1, i32::MIN);
        for num_str in num_strs {
            let index = line.find(num_str);
            if !index.is_none() && ((index.unwrap() as i32) < first_str_index) {
                first_str_index = index.unwrap() as i32;
                first_str = *mappings.get(num_str).unwrap();
            }
            let rindex = line.rfind(num_str);
            if !rindex.is_none() && ((rindex.unwrap() as i32) > last_str_index) {
                last_str_index = rindex.unwrap() as i32;
                last_str = *mappings.get(num_str).unwrap();
            }
        }
        let first = if first_char_index < first_str_index {first_char.to_digit(10).unwrap() as i32} else {first_str};
        let last = if last_char_index > last_str_index {last_char.to_digit(10).unwrap() as i32} else {last_str};
        sum += (10 * first) + last;
    }
    println!("{}", sum);
}