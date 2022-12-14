fn main() {
    // part_one();

    part_two();
}

fn part_one() {
    let input = include_str!("../input.txt")
        .trim_end_matches("\n")
        .split("\n")
        .collect::<Vec<&str>>();

    let mut score = 0;

    for r in input {
        let (p1r, p2r) = r.split_once(" ").unwrap();
        // conver to char
        let p1 = p1r.chars().next().unwrap();
        let p2 = p2r.chars().next().unwrap();

        println!("player one '{}', player two '{}'", p1, p2);

        // shape score
        score += match p2 {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => unimplemented!(),
        };

        match p1 {
            'A' => {
                score += match p2 {
                    'X' => 3,
                    'Y' => 6,
                    'Z' => 0,
                    _ => unimplemented!(),
                }
            }
            'B' => {
                score += match p2 {
                    'X' => 0,
                    'Y' => 3,
                    'Z' => 6,
                    _ => unimplemented!(),
                }
            }
            'C' => {
                score += match p2 {
                    'X' => 6,
                    'Y' => 0,
                    'Z' => 3,
                    _ => unimplemented!(),
                }
            }
            _ => {}
        }
    }

    println!("score: {}", score);
}

fn part_two() {
    let input = include_str!("../input.txt")
        .trim_end_matches("\n")
        .split("\n")
        .collect::<Vec<&str>>();

    let mut score = 0;

    for r in input {
        let (p1r, p2r) = r.split_once(" ").unwrap();
        // conver to char
        let p1 = p1r.chars().next().unwrap();
        let mut p2 = p2r.chars().next().unwrap();

        println!("player one '{}', player two '{}'", p1, p2);

        // A = rock
        // B = paper
        // C = scissors

        // X = rock
        // Y = paper
        // Z = scissors

        p2 = match p2 {
            // lose
            'X' => match p1 {
                'A' => 'Z',
                'B' => 'X',
                'C' => 'Y',
                _ => unimplemented!(),
            },
            // draw
            'Y' => match p1 {
                'A' => 'X',
                'B' => 'Y',
                'C' => 'Z',
                _ => unimplemented!(),
            },
            // win
            'Z' => match p1 {
                'A' => 'Y',
                'B' => 'Z',
                'C' => 'X',
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        };

        // shape score
        score += match p2 {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => 0,
        };

        match p1 {
            'A' => {
                score += match p2 {
                    'X' => 3,
                    'Y' => 6,
                    'Z' => 0,
                    _ => unimplemented!(),
                }
            }
            'B' => {
                score += match p2 {
                    'X' => 0,
                    'Y' => 3,
                    'Z' => 6,
                    _ => unimplemented!(),
                }
            }
            'C' => {
                score += match p2 {
                    'X' => 6,
                    'Y' => 0,
                    'Z' => 3,
                    _ => unimplemented!(),
                }
            }
            _ => {}
        }
    }

    println!("score: {}", score);
}
