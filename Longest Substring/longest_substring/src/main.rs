struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut backup = String::new();
        let mut main_list = String::new();

        let mut iter = s.chars();

        if s.chars().count() == 0 { return 0 }
        else if s.chars().count() == 1 { return 1 };

        backup.push(iter.next().unwrap());

        while let Some(thing) = iter.next() {
            println!("Antes: backup {}, main {}, {thing}\n", backup, main_list);

            if backup.contains(thing) && main_list.chars().count() < backup.chars().count() {
                main_list.clear();
                main_list.push_str(backup.as_str());
                backup.clear();
            } else if backup.contains(thing) { backup.clear(); };
            println!("backup {}, main {}, {thing}\n", backup, main_list);
            
            backup.push(thing);
            println!("Después: backup {}, main {}, {thing}\n", backup, main_list);

            println!("{}", &backup[..1]);
            if main_list.starts_with(&backup[..1]) {
                let letter = main_list.remove(0);
                main_list.push(letter);

                backup.clear();
            }
        }
        main_list.push_str(backup.as_str());
        println!("backup {}, main {}\n", backup, main_list);

        let answer = if main_list.chars().count() > backup.len() { main_list.len() }
        else { backup.chars().count() } as i32;

        answer

    }
}

fn main() {
    assert_eq!(Solution::length_of_longest_substring("dvdf".to_string()), 3);
}