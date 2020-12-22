use std::collections::hash_map::DefaultHasher;
use std::collections::{HashSet, VecDeque};
use std::hash::{Hash, Hasher};

fn deck_from_string(s: &str) -> VecDeque<i32> {
    s.split("\n")
        .skip(1)
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

fn build_decks(filename: &str) -> (VecDeque<i32>, VecDeque<i32>) {
    let groups = common::read_grouped_file(filename);
    (deck_from_string(&groups[0]), deck_from_string(&groups[1]))
}

fn play_game(mut deck1: VecDeque<i32>, mut deck2: VecDeque<i32>) -> (i32, VecDeque<i32>) {
    while !deck1.is_empty() && !deck2.is_empty() {
        let c1 = deck1.pop_front().unwrap();
        let c2 = deck2.pop_front().unwrap();

        if c1 > c2 {
            deck1.push_back(c1);
            deck1.push_back(c2);
        } else {
            deck2.push_back(c2);
            deck2.push_back(c1);
        }
    }

    if deck1.is_empty() {
        (1, deck2)
    } else {
        (0, deck1)
    }
}

fn play_recursive_game(mut deck1: VecDeque<i32>, mut deck2: VecDeque<i32>) -> (i32, VecDeque<i32>) {
    let mut seen_game_states = HashSet::<u64>::new();

    while !deck1.is_empty() && !deck2.is_empty() {
        let mut hasher = DefaultHasher::new();

        (&deck1, &deck2).hash(&mut hasher);
        let state = hasher.finish();
        if seen_game_states.contains(&state) {
            return (0, deck1);
        }
        seen_game_states.insert(state);

        let c1 = deck1.pop_front().unwrap();
        let c2 = deck2.pop_front().unwrap();

        if c1 as usize > deck1.len() || c2 as usize > deck2.len() {
            if c1 > c2 {
                deck1.push_back(c1);
                deck1.push_back(c2);
            } else {
                deck2.push_back(c2);
                deck2.push_back(c1);
            }
        } else {
            let subdeck1 = deck1
                .iter()
                .take(c1 as usize)
                .cloned()
                .collect::<VecDeque<i32>>();
            let subdeck2 = deck2
                .iter()
                .take(c2 as usize)
                .cloned()
                .collect::<VecDeque<i32>>();
            let subgame_result = play_recursive_game(subdeck1, subdeck2);
            if subgame_result.0 == 0 {
                deck1.push_back(c1);
                deck1.push_back(c2);
            } else {
                deck2.push_back(c2);
                deck2.push_back(c1);
            }
        }
    }

    if deck1.is_empty() {
        (1, deck2)
    } else {
        (0, deck1)
    }
}

fn score(deck: VecDeque<i32>) -> i32 {
    deck.iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + ((i + 1) as i32) * x)
}

fn main() {
    let decks = build_decks("src/day22/input.txt");

    let result = play_game(decks.0.clone(), decks.1.clone());
    println!(
        "Player {} wins with score: {}",
        result.0 + 1,
        score(result.1)
    );

    let result = play_recursive_game(decks.0.clone(), decks.1.clone());
    println!(
        "Player {} wins recursive with score: {}",
        result.0 + 1,
        score(result.1)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_normal_game() {
        let decks = build_decks("src/day22/input_test.txt");

        let result = play_game(decks.0, decks.1);
        assert_eq!(
            result,
            (
                1,
                vec![3, 2, 10, 6, 8, 5, 9, 4, 7, 1]
                    .iter()
                    .cloned()
                    .collect::<VecDeque<i32>>()
            )
        );
        assert_eq!(score(result.1), 306);
    }

    #[test]
    fn test_recursive_game_state() {
        let deck1 = vec![43, 19].iter().cloned().collect::<VecDeque<i32>>();
        let deck2 = vec![2, 29, 14].iter().cloned().collect::<VecDeque<i32>>();

        let result = play_recursive_game(deck1, deck2);
        assert_eq!(
            result,
            (0, vec![43, 19].iter().cloned().collect::<VecDeque<i32>>())
        );
    }

    #[test]
    fn test_recursive_game() {
        let decks = build_decks("src/day22/input_test.txt");

        let result = play_recursive_game(decks.0, decks.1);
        assert_eq!(
            result,
            (
                1,
                vec![7, 5, 6, 2, 4, 1, 10, 8, 9, 3]
                    .iter()
                    .cloned()
                    .collect::<VecDeque<i32>>()
            )
        );
        assert_eq!(score(result.1), 291);
    }
}
