struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut list = String::new();

        let mut aux = String::new();

        let mut iter = s.chars();

        if s.chars().count() == 0 {
            return 0;
        } else if s.chars().count() == 1 {
            return 1;
        };

        while let Some(thing) = iter.next() {
            if !aux.contains(thing) {
                aux.push(thing);
            } else {
                let mut letter: char;
                loop {
                    letter = aux.remove(0);

                    if letter == thing {
                        //                        aux.push(letter);
                        break;
                    };
                }
                aux.push(thing);
            }

            if aux.chars().count() > list.chars().count() {
                list = aux.clone();
            }
        }

        let answer = list.chars().count();
        answer as i32
    }
}

fn main() {
    assert_eq!(
        Solution::length_of_longest_substring("abcabcbb".to_string()),
        3
    );
}
