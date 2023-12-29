use aoc23_rust::get_file_contents;

fn main() {
    let contents = get_file_contents("input/day01");
    let mut total = 0;
    contents.iter().for_each(|entry| {
        total += process_entry(entry);
    });
    println!("Part 2: {total}");
}

fn process_entry(entry: &str) -> usize {
    let buffer = fill_buffer(entry);
    parse_buffer(buffer)
}

fn fill_buffer(entry: &str) -> Vec<char> {
    let mut retval: Vec<char> = Vec::new();
    let mut word = String::new();
    entry.chars().for_each(|c| match c {
        '1'..='9' => {
            word.clear();
            retval.push(c);
        }
        _ => {
            word.push(c);
            match parse_word(&word) {
                'x' => (),
                other => {
                    retval.push(other);
                    word.clear();
                },
            }
        }
    });
    retval
}

fn parse_buffer(buffer: Vec<char>) -> usize {
    match buffer.len() {
        0 => 0,
        1 => format!("{0}{0}", buffer.get(0).unwrap()).parse::<usize>().unwrap(),
        _ => format!(
            "{0}{1}",
            buffer.get(0).unwrap(),
            buffer.get(buffer.len() - 1).unwrap()
        )
        .parse::<usize>()
        .unwrap(),
    }
}

fn parse_word(word: &str) -> char {
    let mut retval: char = 'x';
    vec![
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9')
    ].iter().for_each(|e| {
        match e {
            (s, i) => { if word.contains(s) { retval = i.clone(); }},
        }
    });
    retval
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_word() {
        let ipt = "one";
        assert_eq!(parse_word(ipt), '1');
    }

    #[test]
    fn test_fill_buffer() {
        let ipt = "two1nine";
        assert_eq!(vec!['2', '1', '9'], fill_buffer(ipt));

        let ipt = "4nineeightseven2";
        assert_eq!(vec!['4', '9', '8', '7', '2'], fill_buffer(ipt));

        let ipt = "7pqrstsixteen";
        assert_eq!(vec!['7', '6'], fill_buffer(ipt));
    }

    #[test]
    fn test_parse_buffer() {
        let ipt = vec!['2', '1', '9'];
        assert_eq!(29, parse_buffer(ipt));

        let ipt = vec!['4', '9', '8', '7', '2'];
        assert_eq!(42, parse_buffer(ipt));

    }
}
