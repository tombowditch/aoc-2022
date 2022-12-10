fn main() {
    part_one();
}

fn part_one() {
    let input = include_str!("../input.txt")
        .trim_end_matches("\n")
        .split("\n")
        .collect::<Vec<_>>();

    let mut X: isize = 1;
    let mut cycle: isize = 0;
    let mut cycle_counts: Vec<isize> = vec![];
    let mut line: Vec<char> = vec![];

    for instruction in input {
        let op = instruction.split(" ").collect::<Vec<_>>()[0];

        match op {
            "noop" => {
                println!("executing: noop");
                inc_cycle(&mut cycle, X, &mut cycle_counts, &mut line);
            }
            "addx" => {
                println!("executing: addx");
                inc_cycle(&mut cycle, X, &mut cycle_counts, &mut line);
                inc_cycle(&mut cycle, X, &mut cycle_counts, &mut line);
                let ins = instruction.split(" ").collect::<Vec<_>>()[1]
                    .parse::<isize>()
                    .unwrap();

                println!("ins: {}", ins);
                X += ins;
            }
            _ => panic!("unknown instruction"),
        };
    }

    println!("X: {}", X);
    println!("cycle: {}", cycle);

    println!("cycle_counts: {:?}", cycle_counts);
    println!("cycle_counts: {}", cycle_counts.iter().sum::<isize>());

    let split_lines = line.chunks(40);
    for split_line in split_lines {
        println!("{}", split_line.iter().collect::<String>());
    }
}

fn inc_cycle(cycle: &mut isize, X: isize, cycle_counts: &mut Vec<isize>, line: &mut Vec<char>) {
    let CYCLE_CHECKS: Vec<isize> = vec![20, 60, 100, 140, 180, 220];

    let relative_pos = *cycle % 40;
    if vec![X - 1, X, X + 1].contains(&relative_pos) {
        line.push('\u{2588}');
    } else {
        line.push(' ');
    }

    *cycle += 1;

    if CYCLE_CHECKS.contains(&(*cycle as isize)) {
        let ss = *cycle * X;
        println!("signal strengh in cycle {}: {}", cycle, ss);
        cycle_counts.push(ss);
    }
}
