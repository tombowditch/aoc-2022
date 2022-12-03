fn main() {
    part_one();

    part_two();
}

fn get_priority(c: char) -> usize {
    match c {
        'a'..='z' => c as usize - 'a' as usize + 1,
        'A'..='Z' => c as usize - 'A' as usize + 27,
        _ => panic!("Invalid item type"),
    }
}

fn part_one() {
    let input = include_str!("../input.txt")
        .trim_end_matches("\n")
        .split("\n")
        .collect::<Vec<&str>>();

    let mut total_priroity: usize = 0;

    for line in input {
        // split line in half
        let (s1, s2) = line.split_at(line.len() / 2);

        println!("s1: {}", s1);
        println!("s2: {}", s2);

        // find char thats in both s1 and s2
        let s1_chars = s1.chars().collect::<Vec<char>>();
        let s2_chars = s2.chars().collect::<Vec<char>>();

        let mut dup_char: Option<char> = None;

        for c in s1_chars {
            if s2_chars.contains(&c) {
                dup_char = Some(c);
                break;
            }
        }

        println!("dup: {}", dup_char.unwrap());

        total_priroity += get_priority(dup_char.unwrap());
    }

    println!("total: {}", total_priroity);
}

fn part_two() {
    let input = include_str!("../input.txt")
        .trim_end_matches("\n")
        .split("\n")
        .collect::<Vec<&str>>();

    let mut total_priroity: usize = 0;

    // get 3 lines from input at a time in a for loop
    for i in (0..input.len()).step_by(3) {
        let line1 = input[i];
        let line2 = input[i + 1];
        let line3 = input[i + 2];

        println!("line1: {}", line1);
        println!("line2: {}", line2);
        println!("line3: {}", line3);

        let l1_chars = line1.chars().collect::<Vec<char>>();
        let l2_chars = line2.chars().collect::<Vec<char>>();
        let l3_chars = line3.chars().collect::<Vec<char>>();

        let mut dup_char: Option<char> = None;

        for c1 in l1_chars.clone() {
            for c2 in l2_chars.clone() {
                for c3 in l3_chars.clone() {
                    if c1 == c2 && c2 == c3 && c3 == c1 {
                        dup_char = Some(c1);
                        break;
                    }
                }
            }
        }

        println!("dup: {}", dup_char.unwrap());

        total_priroity += get_priority(dup_char.unwrap());
    }

    println!("total: {}", total_priroity);
}

mod tests {
    #[test]
    fn test_priority() {
        assert_eq!(super::get_priority('a'), 1);
        assert_eq!(super::get_priority('z'), 26);
        assert_eq!(super::get_priority('A'), 27);
        assert_eq!(super::get_priority('Z'), 52);
    }
}
