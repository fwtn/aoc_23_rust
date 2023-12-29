fn main() {
    let s = "pqrstsix";
    println!("{0}",parse_word(s));
}

fn parse_word(word: &str) -> char{
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
