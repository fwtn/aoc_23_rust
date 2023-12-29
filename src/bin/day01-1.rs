use aoc23_rust::get_file_contents;

fn main() {
    let contents = get_file_contents("input/day01");
    let mut total = 0;
    //contents.iter().for_each(|entry| {
    //    total += process_day01_entry(entry);
    //});
    //println!("Part 1: {total}");

    //total = 0;
    contents.iter().for_each(|entry| {
        total += process_part2(entry);
    });
    println!("Part 2: {total}");
}

fn _process_day01_entry(entry: &str) -> usize {
    let mut buffer: Vec<char> = Vec::new();
    entry.chars().for_each(|c| {
        match c {
            '1'..='9' => buffer.push(c),
            _ => (),
        };
    });
    match buffer.len() {
        0 => 0,
        1 => format!("{0}{0}", buffer.get(0).unwrap()).parse().unwrap(),
        _ => format!(
            "{0}{1}",
            buffer.get(0).unwrap(),
            buffer.get(buffer.len() - 1).unwrap()
        )
        .parse()
        .unwrap(),
    }
}

fn process_part2(entry: &str) -> usize {
    let mut word = String::new();
    let mut buffer: Vec<char> = Vec::new();
    entry.chars().for_each(|c| match c {
        '1'..='9' => {
            word.clear();
            buffer.push(c);
        }
        _ => {
            word.push(c);
            match parse_word(&word) {
                'x' => (),
                other => {
                    buffer.push(other);
                    word.clear();
                },
            }
        }
    });
    match buffer.len() {
        0 => 0,
        1 => format!("{0}{0}", buffer.get(0).unwrap()).parse().unwrap(),
        _ => format!(
            "{0}{1}",
            buffer.get(0).unwrap(),
            buffer.get(buffer.len() - 1).unwrap()
        )
        .parse()
        .unwrap(),
    };
    0
}

fn parse_word(word: &str) -> char {
    match word {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => 'x',
    }
}
