#![allow(dead_code)]

// Task: Given a QWERTY keyboard grid and a remote control with arrows and a “select” button,
// write a function that returns the buttons you have to press to type a certain word.
// The cursor will always start in the upper left at the letter Q.

use std::fmt::Display;

pub fn remote_control(word: &str) -> String {
    let keyboard: Keyboard<LowerCaseEnglish> = Keyboard::default();
    let mut cursor = keyboard
        .location_of('q')
        .expect("keyboard doesn't have a q key");

    word.chars()
        .fold(vec![], |mut acc, c| {
            let next_location = keyboard
                .location_of(c)
                .expect(format!("keyboard has no {c} key").as_str());

            let instructions = keyboard.go_to(&cursor, &next_location);

            cursor = next_location;

            acc.push(instructions);
            acc
        })
        .into_iter()
        .flatten()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

#[derive(Debug, Copy, Clone)]
enum Instructions {
    Up,
    Down,
    Left,
    Right,
    Select,
}

impl Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Instructions::Up => "up",
            Instructions::Down => "down",
            Instructions::Left => "left",
            Instructions::Right => "right",
            Instructions::Select => "select",
        };

        f.write_str(res)
    }
}

trait KeyMap {
    fn location_of(&self, c: char) -> Option<Location>;
}

#[derive(Default)]
struct LowerCaseEnglish;

impl KeyMap for LowerCaseEnglish {
    fn location_of(&self, c: char) -> Option<Location> {
        match c {
            'q' => Some(Location::at(0, 0)),
            'w' => Some(Location::at(0, 1)),
            'e' => Some(Location::at(0, 2)),
            'r' => Some(Location::at(0, 3)),
            't' => Some(Location::at(0, 4)),
            'y' => Some(Location::at(0, 5)),
            'u' => Some(Location::at(0, 6)),
            'i' => Some(Location::at(0, 7)),
            'o' => Some(Location::at(0, 8)),
            'p' => Some(Location::at(0, 9)),
            'a' => Some(Location::at(1, 0)),
            's' => Some(Location::at(1, 1)),
            'd' => Some(Location::at(1, 2)),
            'f' => Some(Location::at(1, 3)),
            'g' => Some(Location::at(1, 4)),
            'h' => Some(Location::at(1, 5)),
            'j' => Some(Location::at(1, 6)),
            'k' => Some(Location::at(1, 7)),
            'l' => Some(Location::at(1, 8)),
            'z' => Some(Location::at(2, 0)),
            'x' => Some(Location::at(2, 1)),
            'c' => Some(Location::at(2, 2)),
            'v' => Some(Location::at(2, 3)),
            'b' => Some(Location::at(2, 4)),
            'n' => Some(Location::at(2, 5)),
            'm' => Some(Location::at(2, 6)),
            _ => None,
        }
    }
}

#[derive(Default)]
struct Keyboard<T: KeyMap> {
    keys: T,
}

impl<T: KeyMap> Keyboard<T> {
    fn go_to(&self, from: &Location, to: &Location) -> Vec<Instructions> {
        let row_instruction = if to.row > from.row {
            Instructions::Down
        } else {
            Instructions::Up
        };

        let col_instruction = if to.col > from.col {
            Instructions::Right
        } else {
            Instructions::Left
        };

        let mut result: Vec<Instructions> = (0..(from.row - to.row).abs())
            .map(|_| row_instruction)
            .chain((0..(from.col - to.col).abs()).map(|_| col_instruction))
            .collect();

        result.push(Instructions::Select);

        result
    }

    fn location_of(&self, c: char) -> Option<Location> {
        self.keys.location_of(c)
    }
}

struct Key {
    code: char,
    loc: Location,
}

impl Key {
    pub fn new(code: char, loc: Location) -> Self {
        Self { code, loc }
    }
}

#[derive(Default, Debug)]
struct Location {
    row: i32,
    col: i32,
}

impl Location {
    fn at(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn navigates_car() {
        let result = remote_control("car");
        assert_eq!(String::from("down, down, right, right, select, up, left, left, select, up, right, right, right, select"), result);
    }
}
