fn main() {
    // part_one();@

    part_two();
}

fn part_one() {
    let input = include_str!("../input.txt")
        .trim_end_matches("\n")
        .split("\n")
        .collect::<Vec<&str>>();

    let mut count = 0;

    // example: 2-4,6-8
    for line in input {
        let (p1r, p2r) = line.split_once(",").unwrap();

        let (p1start, p1end) = (
            p1r.split_once("-").unwrap().0.parse::<u32>().unwrap(),
            p1r.split_once("-").unwrap().1.parse::<u32>().unwrap(),
        );

        let (p2start, p2end) = (
            p2r.split_once("-").unwrap().0.parse::<u32>().unwrap(),
            p2r.split_once("-").unwrap().1.parse::<u32>().unwrap(),
        );

        let p1 = (p1start..=p1end).collect::<Vec<u32>>();
        let p2 = (p2start..=p2end).collect::<Vec<u32>>();

        println!("{:?} -> {:?}", p1start, p1end);
        println!("{:?} -> {:?}", p2start, p2end);
        println!("-----");

        if p1.iter().all(|item| p2.contains(item)) {
            count += 1;
        } else if p2.iter().all(|item| p1.contains(item)) {
            count += 1;
        }
    }

    println!("count: {}", count);
}

fn part_two() {
    let input = include_str!("../input.txt")
        .trim_end_matches("\n")
        .split("\n")
        .collect::<Vec<&str>>();

    let mut count = 0;

    // example: 2-4,6-8
    for line in input {
        let (p1r, p2r) = line.split_once(",").unwrap();

        let (p1start, p1end) = (
            p1r.split_once("-").unwrap().0.parse::<u32>().unwrap(),
            p1r.split_once("-").unwrap().1.parse::<u32>().unwrap(),
        );

        let (p2start, p2end) = (
            p2r.split_once("-").unwrap().0.parse::<u32>().unwrap(),
            p2r.split_once("-").unwrap().1.parse::<u32>().unwrap(),
        );

        let p1 = (p1start..=p1end).collect::<Vec<u32>>();
        let p2 = (p2start..=p2end).collect::<Vec<u32>>();

        println!("{:?} -> {:?}", p1start, p1end);
        println!("{:?} -> {:?}", p2start, p2end);
        println!("-----");

        let mut d = false;

        for item in p1.clone() {
            if p2.contains(&item) {
                count += 1;
                d = true;
                break;
            }
        }

        if !d {
            for item in p2 {
                if p1.contains(&item) {
                    count += 1;
                    break;
                }
            }
        }

        //
        // if p1.iter().all(|item| p2.contains(item)) {
        //     count += 1;
        // } else if p2.iter().all(|item| p1.contains(item)) {
        //     count += 1;
        // }
    }

    println!("count: {}", count);
}
