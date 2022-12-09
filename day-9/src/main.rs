fn main() {
    // part_one();

    part_two();
}

fn part_one() {
    let input = include_str!("../input.txt")
        .trim_end_matches("\n")
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|c| c.split_once(" ").unwrap())
        .collect::<Vec<(&str, &str)>>();

    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);

    // example instructions
    // R 4
    // U 4
    // L 3
    // D 1

    let mut coord_translations = vec![];

    for raw in input {
        let (dir, dist) = raw;

        let dist = dist.parse::<i32>().unwrap();

        let mut translation = (0, 0);

        match dir {
            "R" => {
                translation = (1, 0);
            }
            "L" => {
                translation = (-1, 0);
            }
            "U" => {
                translation = (0, 1);
            }
            "D" => {
                translation = (0, -1);
            }
            _ => {
                panic!("Invalid direction");
            }
        };

        for _ in 0..dist {
            coord_translations.push(translation);
        }
    }

    println!("translations: {:?}", coord_translations);

    let mut visited: Vec<(i32, i32)> = vec![];

    for translation in coord_translations {
        let (x, y) = translation;

        head_pos = (head_pos.0 + x, head_pos.1 + y);

        let (head_x, head_y) = head_pos;
        let (tail_x, tail_y) = tail_pos;

        // left
        // T H
        if head_x == tail_x + 2 && head_y == tail_y {
            // move 1 right
            tail_pos = (tail_x + 1, tail_y);
        }

        // right
        // H T
        if head_x == tail_x - 2 && head_y == tail_y {
            // move 1 left
            tail_pos = (tail_x - 1, tail_y);
        }

        // up
        // T
        //
        // H
        if head_x == tail_x && head_y == tail_y - 2 {
            // move 1 down
            tail_pos = (tail_x, tail_y - 1);
        }

        // down
        // H
        //
        // T
        if head_x == tail_x && head_y == tail_y + 2 {
            // move 1 up
            tail_pos = (tail_x, tail_y + 1);
        }

        ///////// diagonals

        // bottom left
        //   H
        //T
        //  T
        if head_x == tail_x + 1 && head_y == tail_y + 2
            || head_x == tail_x + 2 && head_y == tail_y + 1
        {
            // move 1 up and 1 right
            tail_pos = (tail_x + 1, tail_y + 1);
        }

        // bottom right
        // H
        //    T
        //  T
        if head_x == tail_x - 1 && head_y == tail_y + 2
            || head_x == tail_x - 2 && head_y == tail_y + 1
        {
            // move 1 up and 1 left
            tail_pos = (tail_x - 1, tail_y + 1);
        }

        // top left
        //  T
        // T
        //   H
        if head_x == tail_x + 1 && head_y == tail_y - 2
            || head_x == tail_x + 2 && head_y == tail_y - 1
        {
            // move 1 down and 1 right
            tail_pos = (tail_x + 1, tail_y - 1);
        }

        // top right
        //   T
        //
        //  H
        if head_x == tail_x - 1 && head_y == tail_y - 2
            || head_x == tail_x - 2 && head_y == tail_y - 1
        {
            // move 1 down and 1 left
            tail_pos = (tail_x - 1, tail_y - 1);
        }

        visited.push(tail_pos);

        println!(
            "head: {:?}, tail: {:?} (taken translation {:?})",
            head_pos, tail_pos, translation
        );
    }

    println!("final - head: {:?}, tail: {:?}", head_pos, tail_pos);

    println!("visited: {:?}", visited);

    visited.sort();
    visited.dedup();

    println!("dedupe visited: {:?}", visited);

    println!("visited places: {}", visited.len());
}

fn part_two() {
    let input = include_str!("../input.txt")
        .trim_end_matches("\n")
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|c| c.split_once(" ").unwrap())
        .collect::<Vec<(&str, &str)>>();

    let mut pos = vec![
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];

    // example instructions
    // R 4
    // U 4
    // L 3
    // D 1

    let mut coord_translations = vec![];

    for raw in input {
        let (dir, dist) = raw;

        let dist = dist.parse::<i32>().unwrap();

        let mut translation = (0, 0);

        match dir {
            "R" => {
                translation = (1, 0);
            }
            "L" => {
                translation = (-1, 0);
            }
            "U" => {
                translation = (0, 1);
            }
            "D" => {
                translation = (0, -1);
            }
            _ => {
                panic!("Invalid direction");
            }
        };

        for _ in 0..dist {
            coord_translations.push(translation);
        }
    }

    println!("translations: {:?}", coord_translations);

    let mut visited: Vec<(i32, i32)> = vec![];

    for translation in coord_translations {
        let (x, y) = translation;

        let mut top_head_pos = pos[0];

        top_head_pos = (top_head_pos.0 + x, top_head_pos.1 + y);
        pos[0] = top_head_pos;

        for i in 1..pos.len() {
            println!("loop");
            let head_pos = pos[i - 1];
            let mut tail_pos = pos[i];

            let (head_x, head_y) = head_pos;
            let (tail_x, tail_y) = tail_pos;

            // left
            // T H
            if head_x == tail_x + 2 && head_y == tail_y {
                // move 1 right
                tail_pos = (tail_x + 1, tail_y);
            }

            // right
            // H T
            if head_x == tail_x - 2 && head_y == tail_y {
                // move 1 left
                tail_pos = (tail_x - 1, tail_y);
            }

            // up
            // T
            //
            // H
            if head_x == tail_x && head_y == tail_y - 2 {
                // move 1 down
                tail_pos = (tail_x, tail_y - 1);
            }

            // down
            // H
            //
            // T
            if head_x == tail_x && head_y == tail_y + 2 {
                // move 1 up
                tail_pos = (tail_x, tail_y + 1);
            }

            ///////// diagonals

            // bottom left
            //   H
            //T
            //  T
            if head_x == tail_x + 1 && head_y == tail_y + 2
                || head_x == tail_x + 2 && head_y == tail_y + 1
                || head_x == tail_x + 2 && head_y == tail_y + 2
            {
                // move 1 up and 1 right
                tail_pos = (tail_x + 1, tail_y + 1);
            }

            // bottom right
            // H
            //    T
            //  T
            if head_x == tail_x - 1 && head_y == tail_y + 2
                || head_x == tail_x - 2 && head_y == tail_y + 1
                || head_x == tail_x - 2 && head_y == tail_y + 2
            {
                // move 1 up and 1 left
                tail_pos = (tail_x - 1, tail_y + 1);
            }

            // top left
            //  T
            // T
            //   H
            if head_x == tail_x + 1 && head_y == tail_y - 2
                || head_x == tail_x + 2 && head_y == tail_y - 1
                || head_x == tail_x + 2 && head_y == tail_y - 2
            {
                // move 1 down and 1 right
                tail_pos = (tail_x + 1, tail_y - 1);
            }

            // top right
            //   T
            //
            //  H
            if head_x == tail_x - 1 && head_y == tail_y - 2
                || head_x == tail_x - 2 && head_y == tail_y - 1
                || head_x == tail_x - 2 && head_y == tail_y - 2
            {
                // move 1 down and 1 left
                tail_pos = (tail_x - 1, tail_y - 1);
            }

            //visited.push(tail_pos);

            println!(
                "head: {:?}, tail: {:?} (taken translation {:?})",
                head_pos, tail_pos, translation
            );

            pos[i] = tail_pos;
        }

        visited.push(pos[pos.len() - 1]);
    }

    println!("{:#?}", pos);

    // println!("final - head: {:?}, tail: {:?}", head_pos, tail_pos);

    println!("visited: {:?}", visited);

    visited.sort();
    visited.dedup();

    println!("dedupe visited: {:?}", visited);

    println!("visited places: {}", visited.len());
}
