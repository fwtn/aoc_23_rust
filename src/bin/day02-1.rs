use aoc23_rust::get_file_contents;

struct GameLine(usize, Vec<String>);

fn main() {
    let input = get_file_contents("input/day02-1-test");
    let mut result = 0;
    input.iter().for_each(|entry| {
        result += process_entry(entry);
    });
}

fn process_entry(entry: &str) -> usize {

    0
}
