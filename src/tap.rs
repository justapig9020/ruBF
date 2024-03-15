use std::collections::VecDeque;

const DEFAULT_TAP_SIZE: usize = 3000;
pub enum Direction {
    Right,
    Left,
}

pub struct Tap {
    cursor: isize,
    tap: VecDeque<u8>,
}

impl Default for Tap {
    fn default() -> Self {
        Self {
            cursor: 0,
            tap: VecDeque::from(vec![0; DEFAULT_TAP_SIZE]),
        }
    }
}

impl Tap {
    pub fn move_cursor(&mut self, direction: Direction) {
        match direction {
            Direction::Left => {
                self.cursor -= 1;
                if self.cursor < 0 {
                    self.tap.push_front(0);
                    self.cursor = 0;
                }
            }
            Direction::Right => {
                self.cursor += 1;
                if self.cursor as usize >= self.tap.len() {
                    self.tap.push_back(0);
                }
            }
        }
    }
    pub fn get(&self) -> u8 {
        let idx = self.cursor as usize;
        self.tap[idx]
    }
    pub fn set(&mut self, value: u8) {
        let idx = self.cursor as usize;
        self.tap[idx] = value;
    }
}

#[cfg(test)]
mod tap {
    use super::*;

    #[test]
    fn test_move_cursor() {
        let mut tap = Tap::default();
        tap.move_cursor(Direction::Right);
        assert_eq!(tap.cursor, 1);
        tap.move_cursor(Direction::Left);
        assert_eq!(tap.cursor, 0);
    }
    #[test]
    fn test_set_and_get() {
        let mut tap = Tap::default();
        tap.set(42);
        assert_eq!(tap.get(), 42);
    }
}
