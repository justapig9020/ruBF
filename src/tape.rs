use std::collections::VecDeque;

const DEFAULT_TAP_SIZE: usize = 10;
pub enum Direction {
    Right,
    Left,
}

#[derive(Debug)]
pub struct Tape {
    cursor: isize,
    tape: VecDeque<u8>,
}

impl Default for Tape {
    fn default() -> Self {
        Self {
            cursor: 0,
            tape: VecDeque::from(vec![0; DEFAULT_TAP_SIZE]),
        }
    }
}

impl Tape {
    pub fn move_cursor(&mut self, direction: Direction) {
        match direction {
            Direction::Left => {
                self.cursor -= 1;
                if self.cursor < 0 {
                    self.tape.push_front(0);
                    self.cursor = 0;
                }
            }
            Direction::Right => {
                self.cursor += 1;
                if self.cursor as usize >= self.tape.len() {
                    self.tape.push_back(0);
                }
            }
        }
    }
    pub fn get(&self) -> u8 {
        let idx = self.cursor as usize;
        self.tape[idx]
    }
    pub fn set(&mut self, value: u8) {
        let idx = self.cursor as usize;
        self.tape[idx] = value;
    }
}

#[cfg(test)]
mod tape {
    use super::*;

    #[test]
    fn test_move_cursor() {
        let mut tape = Tape::default();
        tape.move_cursor(Direction::Right);
        assert_eq!(tape.cursor, 1);
        tape.move_cursor(Direction::Left);
        assert_eq!(tape.cursor, 0);
    }
    #[test]
    fn test_set_and_get() {
        let mut tape = Tape::default();
        tape.set(42);
        assert_eq!(tape.get(), 42);
    }
}
