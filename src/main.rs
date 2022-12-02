fn main() {
    //day_1();
    day_2();
}

fn read_input(day: i32) -> String {
    std::fs::read_to_string(format!("input{day}.txt")).expect("couldn't read input file")
}

fn day_2() {
    let input = read_input(2);
    let lines: Vec<&str> = input.lines().collect();

    let mut score = 0;
    // part 1
    //for line in lines {
    //    let other = line.chars().nth(0).unwrap();
    //    let your = line.chars().nth(2).unwrap();

    //    score += match your {
    //        'X' => 1,
    //        'Y' => 2,
    //        'Z' => 3,
    //        _ => panic!("")
    //    };
    //    
    //    score += match (other, your) {
    //        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
    //        ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
    //        _ => 0
    //    };
    //}
    
    for line in lines {
        let other = line.chars().nth(0).unwrap();
        let your = line.chars().nth(2).unwrap();

        score += match your {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => panic!()
        };

        score += match (other, your) {
            ('A', 'X') | ('B', 'Z') | ('C', 'Y') => 3,
            ('A', 'Y') | ('B', 'X') | ('C', 'Z') => 1,
            _ => 2
        };
    }
    println!("{}", score);
}

fn day_1() {
    let input = read_input(1);

    let mut calories: Vec<i32> = vec![];
    let mut current = 0;

    for line in input.lines() {
        if line == "" {
            calories.push(current);
            current = 0;
        }
        else {
            current += line.parse::<i32>().unwrap();
        }
    }

    let mut max = [0; 3];
    for cal in calories {
        if cal > max[0] {
            max[2] = max[1];
            max[1] = max[0];
            max[0] = cal;
        }
        else if cal > max[1] {
            max[2] = max[1];
            max[1] = cal;
        }
        else if cal > max[2] {
            max[2] = cal;
        }
    }
    // just use max[0] for part 1
    println!("{}", max[0] + max[1] + max[2]);
}
