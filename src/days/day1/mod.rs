use std::{collections::HashMap, fs, str};


pub fn day1() {
    println!("\n 👉 Day 1:");
    let file_path_p1 = "src/data/day1/input.txt";
    let content_part_1 = get_content(file_path_p1);
    let part1_result = day1_part1(content_part_1);
    println!("\t✅ Part 1: {}", part1_result);

    let file_path_p2 = "src/data/day1/input2.txt";
    let content_part_2 = get_content(file_path_p2);
    let part2_result = day1_part2(content_part_2);
    println!("\t✅ Part 2: {}", part2_result);
}

fn get_content(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn day1_part1(content: String) -> i32 {
    let lines: Vec<&str> = content.split("\n").collect();
    let mut final_nums: Vec<i32> = vec![];
    for line in lines {
        let mut numbers: Vec<i32> = vec![];
        for char in line.chars() {
            let num = char.to_string().parse::<i32>();
            match num {
                Ok(n) => numbers.push(n),
                Err(_) => ()
            }
        }
        
        if numbers.len() >= 2 {
            let fist_num = numbers[0];
            let last_num = numbers[numbers.len() - 1];
            let combined: i32 = format!("{0}{1}", fist_num, last_num).parse().unwrap();
            final_nums.push(combined)
        } else if numbers.len() == 1 {
            let combined: i32 = format!("{0}{1}", numbers[0], numbers[0]).parse().unwrap();
            final_nums.push(combined)
        }
    }
    
    final_nums.iter().sum()
}


fn day1_part2(content: String) -> i32 {
    let lines: Vec<&str> = content.split("\n").collect();
    let mut final_nums: Vec<i32> = vec![];
    let num_in_letters: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
        ("zero", "0"),
    ]);
    for line in lines {
        let mut line_values: Vec<(usize, &str)> = vec![];
        for (key, value) in num_in_letters.iter() {
            let key_indices: Vec<(usize, &str)> = line.match_indices(key).collect();
            if key_indices.len() > 1 {
                line_values.push(key_indices[0]);
                line_values.push(key_indices[key_indices.len() - 1]);
            } else if key_indices.len() == 1 {
                line_values.push(key_indices[0]);
            }

            let value_indices: Vec<(usize, &str)> = line.match_indices(*value).collect();
            if value_indices.len() > 1 {
                line_values.push(value_indices[0]);
                line_values.push(value_indices[value_indices.len() - 1]);
            } else if value_indices.len() == 1 {
                line_values.push(value_indices[0]);
            } else {
                continue;
            }
        }

        line_values.sort_by(|l1, l2| l1.0.partial_cmp(&l2.0).unwrap());


        if line_values.len() >= 2 {
            let maybe_first_num = line_values[0].1;
            let maybe_in_letters = num_in_letters.get(maybe_first_num);
            let fist_num = match maybe_in_letters {
                Some(val) => *val,
                None => maybe_first_num,
            };

            let maybe_last_num = line_values[line_values.len() - 1].1;
            let maybe_in_letters_too = num_in_letters.get(maybe_last_num);
            let last_num = match maybe_in_letters_too {
                Some(val) => *val,
                None => maybe_last_num,
            };
            let combined: i32 = format!("{0}{1}", fist_num, last_num).parse().unwrap();
            final_nums.push(combined);
        } else if line_values.len() == 1 {
            let maybe_num = line_values[0].1;
            let maybe_letters = num_in_letters.get(maybe_num);
            let num = match maybe_letters {
                Some(val) => *val,
                None => maybe_num,
            };
            let combined: i32 = format!("{0}{1}", num, num).parse().unwrap();
            final_nums.push(combined);
        }
    }

    final_nums.iter().sum()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{day1_part1, day1_part2};

    #[test]
    fn test_day1_part1() {
        let file_path = "src/data/day1/test.txt";
        let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
        let result = day1_part1(contents);

        assert_eq!(result, 142);
    }

    #[test]
    fn test_day1_part2() {
        let file_path = "src/data/day1/test2.txt";
        let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
        let result = day1_part2(contents);

        assert_eq!(result, 281);
    }
}