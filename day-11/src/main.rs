use eval::eval;
use num::integer::lcm;

fn main() {
    //part_one();

    part_two();
}

#[derive(Debug, Clone)]
pub struct Monkey {
    pub number: isize,
    pub starting_items: Vec<isize>,
    pub raw_operation: String,
    pub division_test: isize,
    pub true_monkey: isize,
    pub false_monkey: isize,
    pub inspections: isize,
}

fn part_one() {
    let input = include_str!("../input.txt");

    let monkey_raw_blocks = input.split("\n\n").collect::<Vec<&str>>();

    let mut monkeys = Vec::new();

    for mrb in monkey_raw_blocks {
        let lines = mrb.split("\n").collect::<Vec<&str>>();

        let num = lines[0].split(" ").collect::<Vec<&str>>()[1]
            .trim_end_matches(":")
            .parse::<isize>()
            .unwrap();

        println!("monkey {} got", num);

        let starting = lines[1].split(": ").collect::<Vec<_>>()[1]
            .split(", ")
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<_>>();

        let raw_operation =
            lines[2].split(": ").collect::<Vec<_>>()[1].trim_start_matches("new = ");

        let raw_test = lines[3].split(": ").collect::<Vec<_>>()[1]
            .trim_start_matches("divisible by ")
            .parse::<isize>()
            .unwrap();

        let raw_test_true = lines[4].split(": ").collect::<Vec<_>>()[1]
            .trim_start_matches("throw to monkey ")
            .parse::<isize>()
            .unwrap();

        let raw_test_false = lines[5].split(": ").collect::<Vec<_>>()[1]
            .trim_start_matches("throw to monkey ")
            .parse::<isize>()
            .unwrap();

        let monkey = Monkey {
            number: num,
            starting_items: starting,
            raw_operation: raw_operation.to_string(),
            division_test: raw_test,
            true_monkey: raw_test_true,
            false_monkey: raw_test_false,
            inspections: 0,
        };

        monkeys.push(monkey);
    }

    println!("{:#?}", monkeys);

    for round in 1..=20 {
        println!("executing round {}", round);

        for m in 0..monkeys.len() {
            let monkey = monkeys[m].clone();
            println!("processing monkey {}", monkey.number);

            let inspections = monkey.starting_items.len();

            for i in 0..monkey.starting_items.len() {
                let mut item = monkey.starting_items[i];

                println!("processing item {}", item);

                let operation = monkey.raw_operation.replace("old", &item.to_string());

                let res = eval(&operation)
                    .unwrap()
                    .to_string()
                    .parse::<isize>()
                    .unwrap();

                println!("result of operation {} is {}", operation, res);

                item = res;

                item = item / 3;

                println!("itm: {}", item);

                let tm = monkey.true_monkey.clone();
                let fm = monkey.false_monkey.clone();

                if item % monkey.division_test == 0 {
                    monkeys[tm as usize].starting_items.push(item);
                } else {
                    monkeys[fm as usize].starting_items.push(item);
                }
            }

            monkeys[m].starting_items = Vec::new();
            monkeys[m].inspections += inspections as isize;
        }

        println!("ROUND {}", round);
        for m in monkeys.clone() {
            println!(
                "monkey {} has {} items: {:?}",
                m.number,
                m.starting_items.len(),
                m.starting_items
            );
            println!("{} inspections", m.inspections);
        }

        // find highest 2 inspections
        let mut ins = monkeys
            .clone()
            .iter()
            .map(|m| m.inspections)
            .collect::<Vec<_>>();
        ins.sort();

        let highest = ins[ins.len() - 1];
        let second_highest = ins[ins.len() - 2];

        println!("res: {}", highest * second_highest);
    }
}

fn part_two() {
    let input = include_str!("../input.txt");

    let monkey_raw_blocks = input.split("\n\n").collect::<Vec<&str>>();

    let mut monkeys = Vec::new();

    for mrb in monkey_raw_blocks {
        let lines = mrb.split("\n").collect::<Vec<&str>>();

        let num = lines[0].split(" ").collect::<Vec<&str>>()[1]
            .trim_end_matches(":")
            .parse::<isize>()
            .unwrap();

        println!("monkey {} got", num);

        let starting = lines[1].split(": ").collect::<Vec<_>>()[1]
            .split(", ")
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<_>>();

        let raw_operation =
            lines[2].split(": ").collect::<Vec<_>>()[1].trim_start_matches("new = ");

        let raw_test = lines[3].split(": ").collect::<Vec<_>>()[1]
            .trim_start_matches("divisible by ")
            .parse::<isize>()
            .unwrap();

        let raw_test_true = lines[4].split(": ").collect::<Vec<_>>()[1]
            .trim_start_matches("throw to monkey ")
            .parse::<isize>()
            .unwrap();

        let raw_test_false = lines[5].split(": ").collect::<Vec<_>>()[1]
            .trim_start_matches("throw to monkey ")
            .parse::<isize>()
            .unwrap();

        let monkey = Monkey {
            number: num,
            starting_items: starting,
            raw_operation: raw_operation.to_string(),
            division_test: raw_test,
            true_monkey: raw_test_true,
            false_monkey: raw_test_false,
            inspections: 0,
        };

        monkeys.push(monkey);
    }

    println!("{:#?}", monkeys);

    let mut n: Vec<isize> = Vec::new();
    for m in monkeys.clone() {
        n.push(m.division_test)
    }

    // let l = n.iter().fold(1, |acc, x| lcm(acc, *x));
    let l = n.iter().product::<isize>();

    for round in 1..=10000 {
        println!("executing round {}", round);

        for m in 0..monkeys.len() {
            let monkey = monkeys[m].clone();
            println!("processing monkey {}", monkey.number);

            let inspections = monkey.starting_items.len();

            for i in 0..monkey.starting_items.len() {
                let mut item = monkey.starting_items[i];

                println!("processing item {}", item);

                let operation = monkey.raw_operation.replace("old", &item.to_string());

                let res = eval(&operation)
                    .unwrap()
                    .to_string()
                    .parse::<isize>()
                    .unwrap();

                println!("result of operation {} is {}", operation, res);

                item = res;

                item = item % l;

                println!("itm: {}", item);

                let tm = monkey.true_monkey.clone();
                let fm = monkey.false_monkey.clone();

                if item % monkey.division_test == 0 {
                    monkeys[tm as usize].starting_items.push(item);
                } else {
                    monkeys[fm as usize].starting_items.push(item);
                }
            }

            monkeys[m].starting_items = Vec::new();
            monkeys[m].inspections += inspections as isize;
        }

        println!("ROUND {}", round);
        for m in monkeys.clone() {
            println!(
                "monkey {} has {} items: {:?}",
                m.number,
                m.starting_items.len(),
                m.starting_items
            );
            println!("{} inspections", m.inspections);
        }

        // find highest 2 inspections
        let mut ins = monkeys
            .clone()
            .iter()
            .map(|m| m.inspections)
            .collect::<Vec<_>>();
        ins.sort();

        let highest = ins[ins.len() - 1];
        let second_highest = ins[ins.len() - 2];

        println!("res: {}", highest * second_highest);
    }
}
