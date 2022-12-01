use std::fs;

fn main() {
    // part_one();

    part_two();
}

fn part_one() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let chunk = input_str.split("\n\n").collect::<Vec<&str>>();

    let mut highest = 0;

    for c in chunk {
        // println!("raw chunk: {:?}", c);
        // remove leading \n at end
        let c = c.trim_end_matches("\n");
        let total_cals = c
            .split("\n")
            .map(|v| v.parse().unwrap())
            .collect::<Vec<i64>>()
            .iter()
            .sum::<i64>();

        println!("chunk: {:?}", total_cals);

        if total_cals > highest {
            highest = total_cals;
        }
    }

    println!("highest: {:?}", highest);
}

fn part_two() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let chunk = input_str.split("\n\n").collect::<Vec<&str>>();

    let mut total_cal_vec: Vec<i64> = Vec::new();

    let mut highest = 0;

    for c in chunk {
        // println!("raw chunk: {:?}", c);
        // remove leading \n at end
        let c = c.trim_end_matches("\n");
        let total_cals = c
            .split("\n")
            .map(|v| v.parse().unwrap())
            .collect::<Vec<i64>>()
            .iter()
            .sum::<i64>();

        println!("chunk: {:?}", total_cals);

        total_cal_vec.push(total_cals);
    }

    total_cal_vec.sort();
    total_cal_vec.reverse();

    println!("tcv: {:?}", total_cal_vec);

    println!(
        "top 3 : {:?}",
        (total_cal_vec[0] + total_cal_vec[1] + total_cal_vec[2])
    );
}
