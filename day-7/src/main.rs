use std::collections::HashMap;

fn main() {
    part_one();
}

#[derive(Debug, Clone)]
struct Directory {
    path: String,
    children: HashMap<String, Directory>,
    files: Vec<File>,
    total: Option<usize>,
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: u64,
}

fn sum_dir(dir: Directory) -> usize {
    let mut tot = 0;
    for f in dir.files {
        tot += f.size as usize;
    }

    for (_, d) in dir.children {
        tot += sum_dir(d);
    }

    tot
}

fn part_one() {
    let input = include_str!("../input.txt")
        .trim_end_matches("\n")
        .split("\n")
        .collect::<Vec<&str>>();

    let mut root = Directory {
        path: String::from("/"),
        children: HashMap::new(),
        files: Vec::new(),
        total: None,
    };

    let mut current_dir: String = "/".to_string();

    for line in input {
        if line.starts_with("$ ") {
            if line.starts_with("$ cd") {
                let path = line.split(" ").collect::<Vec<&str>>()[2];
                let mut tcd = current_dir.clone();
                if tcd == "/" {
                    tcd = "".to_string();
                }

                if path == "/" {
                    current_dir = "/".to_string();
                } else if path == ".." {
                    current_dir = tcd.split("/").collect::<Vec<&str>>()
                        [0..tcd.split("/").collect::<Vec<&str>>().len() - 1]
                        .join("/");

                    if current_dir == "" {
                        current_dir = "/".to_string();
                    }
                } else {
                    let f = format!("{}/{}", tcd.clone(), path).clone();
                    current_dir = f;
                }
            }
        }

        // get current directory as dir
        let mut dir = &mut root;
        for p in current_dir.split("/").collect::<Vec<&str>>() {
            if p == "" {
                continue;
            }
            dir = dir.children.entry(p.to_string()).or_insert(Directory {
                path: p.to_string(),
                children: HashMap::new(),
                files: Vec::new(),
                total: None,
            });
        }

        println!(
            "current dir : {} // dir path : {} // raw line : {}",
            current_dir, dir.path, line
        );

        // process rest of command / file / etc

        if line.starts_with("$ ") {
            continue;
        }

        if line.starts_with("dir ") {
            continue;
        }

        let spl = line.split(" ").collect::<Vec<&str>>();

        let size = spl[0].parse::<usize>().unwrap();
        let name = spl[1].to_string();

        dir.files.push(File {
            name,
            size: size as u64,
        });
    }

    // calculate total on all directories
    calc_total(&mut root);

    println!("{:#?}", root);

    let mut t = 0;
    let mut smallest = 0;

    let needed = 30000000 - (70000000 - root.total.unwrap());

    println!("needed {}", needed);

    sum_tot(&root, &mut t, &mut smallest, needed);

    println!("total : {}", t);
    println!("smallest : {}", smallest);
}

fn calc_total(dir: &mut Directory) {
    let mut tot = 0;
    for f in dir.files.iter_mut() {
        tot += f.size as usize;
    }

    for (_, d) in dir.children.iter_mut() {
        calc_total(d);
        tot += sum_dir(d.clone());
    }

    dir.total = Some(tot);
}

fn sum_tot(dir: &Directory, t: &mut usize, smallest: &mut usize, needed: usize) {
    for d in dir.children.values() {
        sum_tot(d, t, smallest, needed);

        println!("{} : {}", d.path, d.total.unwrap());
        if d.total.unwrap() > needed {
            if *smallest == 0 {
                *smallest = d.total.unwrap();
            } else if d.total.unwrap() < *smallest {
                *smallest = d.total.unwrap();
            }
        }

        if d.total.unwrap() <= 100000 {
            *t += d.total.unwrap();
        }
    }
}
