use std::collections::HashMap;

fn main() {
    // go(false);

    go(true)
}

fn go(part_two: bool) {
    let input = include_str!("../input.txt").trim_end_matches("\n");
    let split = input.split("\n\n").collect::<Vec<&str>>();

    let instructions = split[1].split("\n").collect::<Vec<&str>>();

    // first parse bottom line into number + string index
    let og = split[0];
    let mut og_split = og.split("\n").collect::<Vec<&str>>();
    let b_line = og_split[og_split.len() - 1];
    og_split.pop();
    println!("bottom line: `{}`", b_line);

    let stacks = b_line.replace(" ", "").chars().collect::<Vec<char>>().len();
    println!("stacks: {}", stacks);

    let mut stack_char_idx_map: HashMap<char, usize> = HashMap::new();

    for (i, c) in b_line.chars().collect::<Vec<char>>().iter().enumerate() {
        println!("{}: {}", i, c);
        if *c == ' ' {
            continue;
        }

        stack_char_idx_map.insert(*c, i);
    }

    println!("stack_char_idx_map: {:?}", stack_char_idx_map);

    let mut stack_map: HashMap<char, Vec<char>> = HashMap::new();

    for stack_line in og_split {
        for (c, idx) in stack_char_idx_map.iter() {
            let stack_line_chars = stack_line.chars().collect::<Vec<char>>();
            if stack_line_chars[*idx] != ' ' {
                stack_map
                    .entry(*c)
                    .or_insert(Vec::new())
                    .push(stack_line_chars[*idx]);
            }
        }
    }

    for (k, v) in stack_map.clone().iter() {
        stack_map.entry(*k).or_insert(Vec::new()).reverse();
    }

    println!("stack_map:\n{:#?}", stack_map);

    // parse and execute instructions
    //
    // example instructions:
    // move 1 from 2 to 1
    // move 3 from 1 to 3

    for i in instructions {
        let ins_split = i.split(" ").collect::<Vec<&str>>();
        let count = ins_split[1].parse::<usize>().unwrap();
        let from = ins_split[3].parse::<char>().unwrap();
        let to = ins_split[5].parse::<char>().unwrap();

        println!(
            "moving {} stack(s) from stack {} to stack {}",
            count, from, to
        );

        let mut tmp_stack = Vec::new();

        for _ in 0..count {
            let c = stack_map.get_mut(&from).unwrap().pop().unwrap();
            tmp_stack.push(c);
        }

        if !part_two {
            tmp_stack.reverse();
        }

        for _ in 0..count {
            let c = tmp_stack.pop().unwrap();
            stack_map.get_mut(&to).unwrap().push(c);
        }

        println!("stack_map:\n{:#?}", stack_map);
    }

    println!("FINAL stack_map:\n{:#?}", stack_map);

    let mut final_msg: Vec<char> = Vec::new();

    for i in 1..=stacks {
        // final_msg.push(
        let top_char = stack_map
            .get_mut(&i.to_string().chars().collect::<Vec<char>>()[0])
            .unwrap()
            .pop()
            .unwrap();

        final_msg.push(top_char);
    }

    println!("final msg: {}", final_msg.iter().collect::<String>());
}
