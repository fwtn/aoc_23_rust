use std::collections::HashMap;

use aoc23_rust::get_file_contents;
use velcro::hash_map;

type Color = String;
type DiceCount = usize;
type GameNumber = usize;
type Hand = HashMap<Color, DiceCount>;
type Game = (GameNumber, Vec<Hand>);

fn main() {
    let input = get_file_contents("input/day02-1-test");
    let mut result = 0;
    input.iter().for_each(|entry| {
        result += process_entry(entry);
    });
}

fn process_entry(entry: &str) -> usize {
    let _game_line = extract_game(entry);
    0
}

fn extract_game(entry: &str) -> Game {
    let mut sp = entry.split(':');
    let game_number = sp.next().unwrap()
        .to_string()
        .replace("Game ", "")
        .parse().unwrap();
    let _hands = sp.next().unwrap().split(',').into_iter().map(|e|{
        e.to_string().remove(0)
    });
    (game_number , vec![hash_map!("red".to_string(): 4 as usize)])
}


#[cfg(test)]
mod tests {
    use velcro::vec;

    use super::*;

    #[test]
    pub fn test_extact_game() {
        let ipt = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = (
            1, vec!(
                hash_map!(
                    "blue".to_string(): 3 as usize,
                    "red".to_string(): 4 as usize,),
                hash_map!(
                    "red".to_string(): 1 as usize,
                    "green".to_string(): 2 as usize,
                    "blue".to_string(): 6 as usize,),
                hash_map!(
                    "green".to_string(): 2 as usize)
            )
        );
        assert_eq!(extract_game(ipt), expected);
    }
}
