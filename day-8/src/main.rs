fn main() {
    part_one();

    part_two();
}

fn part_one() {
    let input = include_str!("../input.txt");
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut trees = 0;

    // count all edges and add to trees
    trees += grid.len() * 2; // Y down
    trees += grid[0].len() * 2; // X right
    trees -= 4; // remove the corners, duplicated

    println!("Trees: {}", trees);

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            // if coords are on the edges, skip
            if x == 0 || x == row.len() - 1 || y == 0 || y == grid.len() - 1 {
                continue;
            }

            println!("{} {}: {}", x, y, cell);

            // check all items above cell
            let mut ok = true;
            for i in 0..=(y - 1) {
                let check = grid[i][x];
                if check >= *cell {
                    ok = false;
                    break;
                }
            }
            if ok {
                trees += 1;
                continue;
            }

            // check all items below cell
            let mut ok = true;
            for i in (y + 1)..grid.len() {
                let check = grid[i][x];
                if check >= *cell {
                    ok = false;
                    break;
                }
            }
            if ok {
                trees += 1;
                continue;
            }

            // check all items left of cell
            let mut ok = true;
            for i in 0..=(x - 1) {
                let check = grid[y][i];
                if check >= *cell {
                    ok = false;
                    break;
                }
            }

            if ok {
                trees += 1;
                continue;
            }

            // check all items right of cell
            let mut ok = true;
            for i in (x + 1)..row.len() {
                let check = grid[y][i];
                if check >= *cell {
                    ok = false;
                    break;
                }
            }

            if ok {
                trees += 1;
                continue;
            }
        }
    }

    println!("Trees: {}", trees);
}

fn part_two() {
    let input = include_str!("../reece.txt");
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut trees = 0;

    // count all edges and add to trees
    trees += grid.len() * 2; // Y down
    trees += grid[0].len() * 2; // X right
    trees -= 4; // remove the corners, duplicated

    println!("Trees: {}", trees);

    let mut scores = vec![];

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            // if coords are on the edges, skip
            if x == 0 || x == row.len() - 1 || y == 0 || y == grid.len() - 1 {
                continue;
            }

            println!("{} {}: {}", x, y, cell);

            let mut counts = vec![];

            // check all items above cell
            let mut count = 0;
            for i in (0..=(y - 1)).rev() {
                let check = grid[i][x];
                count += 1;
                if check >= *cell {
                    break;
                }
            }
            counts.push(count);
            println!("up {}", count);

            // check all items below cell
            let mut count = 0;
            for i in (y + 1)..grid.len() {
                let check = grid[i][x];
                count += 1;
                if check >= *cell {
                    break;
                }
            }
            counts.push(count);
            println!("down {}", count);

            // check all items left of cell
            let mut count = 0;
            for i in (0..=(x - 1)).rev() {
                let check = grid[y][i];
                println!("grid[{y}][{i}] = {check}");
                count += 1;
                if check >= *cell {
                    break;
                }
            }
            counts.push(count);
            println!("left {}", count);

            // check all items right of cell
            let mut count = 0;
            for i in (x + 1)..row.len() {
                let check = grid[y][i];
                count += 1;
                if check >= *cell {
                    break;
                }
            }
            counts.push(count);
            println!("right {}", count);

            println!("Counts: {:?}", counts);

            let count = counts.iter().product::<u32>();

            println!("Count: {}", count);

            scores.push(count);
        }
    }

    println!("Scores: {:?}", scores);

    let max = scores.iter().max().unwrap();

    println!("Max: {}", max);
}
