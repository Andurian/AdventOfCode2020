use std::collections::HashMap;

struct Game {
    last_number: u32,
    history: HashMap<u32, Vec<u32>>,
    current_turn: u32,
}

impl Game {
    fn new(init: &Vec<u32>) -> Game {
        let mut game = Game {
            last_number: 0,
            history: HashMap::<u32, Vec<u32>>::new(),
            current_turn: 1,
        };
        for n in init {
            game.add_number(*n)
        }
        game
    }

    fn add_number(&mut self, number: u32) {
        match self.history.get_mut(&number) {
            Some(n) => n.push(self.current_turn),
            None => {
                self.history.insert(number, vec![self.current_turn]);
            }
        }
        self.last_number = number;
        self.current_turn += 1;
    }

    fn get_number(&self) -> u32 {
        let n = self.history.get(&self.last_number).unwrap();
        match n.len() {
            x if x == 1 => 0,
            x if x > 1 => self.current_turn - 1 - n[n.len() - 2],
            _ => panic!("shouldn't happen"),
        }
    }

    fn next(&mut self) -> u32 {
        let n = self.get_number();
        self.add_number(n);
        n
    }

    fn advance_to(&mut self, turn: u32) -> u32 {
        if turn < self.current_turn {
            panic!("turn too low")
        }

        while self.current_turn < turn {
            self.next();
        }

        self.get_number()
    }
}

fn main() {
    println!("Solution 1: {}", Game::new(&vec![0,3,1,6,7,5]).advance_to(2020));
    println!("Solution 2: {}", Game::new(&vec![0,3,1,6,7,5]).advance_to(30_000_000));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_game() {
        assert_eq!(Game::new(&vec![0, 3, 6]).advance_to(2020), 436);
        assert_eq!(Game::new(&vec![1, 3, 2]).advance_to(2020), 1);
        assert_eq!(Game::new(&vec![2, 1, 3]).advance_to(2020), 10);
        assert_eq!(Game::new(&vec![1, 2, 3]).advance_to(2020), 27);
        assert_eq!(Game::new(&vec![2, 3, 1]).advance_to(2020), 78);
        assert_eq!(Game::new(&vec![3, 2, 1]).advance_to(2020), 438);
        assert_eq!(Game::new(&vec![3, 1, 2]).advance_to(2020), 1836);
    }
}
