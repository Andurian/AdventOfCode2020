fn transform_subject_number(mut curr: u128, subject_number: u128) -> u128{
    curr *= subject_number;
    curr % 20201227
}

fn find_loop_size(subject_number: u128, target: u128) -> u128{
    let mut loop_size = 0u128;
    let mut curr = 1;
    loop {
        if curr == target{
            return loop_size;
        }
        curr = transform_subject_number(curr, subject_number);
        loop_size += 1;
    }
}

fn find_encryption_key(subject_number: u128, public_key_door: u128, public_key_card: u128) -> u128{
    let loop_size_door = find_loop_size(subject_number, public_key_door);
    let mut key = 1;
    println!("LSD: {}", loop_size_door);
    for _ in 0..loop_size_door{
        key = transform_subject_number(key, public_key_card);
    }
    key
}

fn main(){
   println!("Key: {}", find_encryption_key(7, 6929599, 2448427));

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encryption_key() {
        assert_eq!(find_encryption_key(7, 17807724, 5764801), 14897079);
    }
}