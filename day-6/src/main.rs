fn main() {
    // part_one();

    // part_two();

    get_start_packet(4);

    get_start_packet(14);
}

fn get_start_packet(uniq: usize) {
    // let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    // let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let input = include_str!("../input.txt");

    let chars = input.chars().collect::<Vec<_>>();

    for (i, c) in chars.iter().enumerate() {
        let mut got_chars = vec![];
        let mut fail = false;

        for ii in 0..uniq {
            if got_chars.contains(&chars[i + ii]) {
                fail = true;
                break;
            }

            got_chars.push(chars[i + ii]);
        }

        if !fail {
            println!("{}: {}", i + uniq, got_chars.iter().collect::<String>());
            break;
        }

        //
        // let (c1, c2, c3, c4) = (chars[i], chars[i + 1], chars[i + 2], chars[i + 3]);
        //
        // // check if c1 c2 c3 c4 are different to each other
        // if c1 != c2 && c1 != c3 && c1 != c4 && c2 != c3 && c2 != c4 && c3 != c4 {
        //     println!("{}{}{}{}", c1, c2, c3, c4);
        //
        //     println!("idx {}", i + 4);
        //     break;
        // }
    }
}

// lazy !!!!
fn part_one() {
    // let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    // let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let input = include_str!("../input.txt");

    let chars = input.chars().collect::<Vec<_>>();

    for (i, c) in chars.iter().enumerate() {
        let (c1, c2, c3, c4) = (chars[i], chars[i + 1], chars[i + 2], chars[i + 3]);

        // check if c1 c2 c3 c4 are different to each other
        if c1 != c2 && c1 != c3 && c1 != c4 && c2 != c3 && c2 != c4 && c3 != c4 {
            println!("{}{}{}{}", c1, c2, c3, c4);

            println!("idx {}", i + 4);
            break;
        }
    }
}

// this isnt going to work !!
fn part_two() {
    let input = include_str!("../input.txt");

    let chars = input.chars().collect::<Vec<_>>();

    for (i, c) in chars.iter().enumerate() {
        let (c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14) = (
            chars[i],
            chars[i + 1],
            chars[i + 2],
            chars[i + 3],
            chars[i + 4],
            chars[i + 5],
            chars[i + 6],
            chars[i + 7],
            chars[i + 8],
            chars[i + 9],
            chars[i + 10],
            chars[i + 11],
            chars[i + 12],
            chars[i + 13],
        );

        // check if c1 c2 c3 c4 c5 c6 c7 c8 c9 c10 c11 c12 c13 c14 are different to each other

        if c1 != c2 && c1 != c3 && c1 != c4 && c2 != c3 && c2 != c4 && c3 != c4 {
            println!("{}{}{}{}", c1, c2, c3, c4);

            println!("idx {}", i + 4);
            break;
        }
    }
}
