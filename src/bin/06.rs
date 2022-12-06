use std::{collections::HashSet, fs};

pub fn find_marker(chars: &Vec<char>) -> u32 {
    let mut iteration = 0_u32;
    for chars in chars.windows(4) {
        iteration = iteration + 1;
        dbg!(chars);
        match chars {
            [a, b, c, d] => {
                let hash = HashSet::from([a, b, c, d]);
                if hash.len() == 4 {
                    dbg!(a, b, c, d);
                    return iteration + 3;
                }
            }
            _ => panic!("Marker not found"),
        }
    }
    0
}

pub fn main() {
    let content = fs::read_to_string("./inputs/06.txt").unwrap();
    let chars = content.chars().collect::<Vec<char>>();
    let res = find_marker(&chars);
    dbg!(res);
}

#[cfg(test)]
#[test]
fn it_should_find_the_marker() {
    assert_eq!(
        7_u32,
        find_marker(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars().collect::<Vec<_>>())
    );
    assert_eq!(
        5_u32,
        find_marker(&"bvwbjplbgvbhsrlpgdmjqwftvncz".chars().collect::<Vec<_>>())
    );
    assert_eq!(
        11_u32,
        find_marker(
            &"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
                .chars()
                .collect::<Vec<_>>()
        )
    );
}
