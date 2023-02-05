struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        println!("Palabra: {}", s);
        let mut list = String::new();

        let mut aux = String::new();

        let mut iter = s.chars();

        if s.chars().count() == 0 {
            return 0;
        } else if s.chars().count() == 1 {
            return 1;
        };

        while let Some(thing) = iter.next() {
            println!("thing: {}", thing);
            if !aux.contains(thing) {
                aux.push(thing);
                println!("no contains, ahora aux: {}\n", aux);
            } else {
                println!("sí contains, al principio aux: {}", aux);
                /*for index in 0..(aux.rfind(thing).unwrap()) {
                    println!("Índice: {}", index);
                    let c = aux.remove(index);
                    println!("Se removió: {}", c);
                }*/
                let mut letter: char;
                loop {
                    letter = aux.remove(0);

                    if letter == thing {
                        //                        aux.push(letter);
                        break;
                    };
                }
                aux.push(thing);
                println!("sí contains, al final aux: {}\n", aux);
            }

            if aux.chars().count() > list.chars().count() {
                list = aux.clone();
            }
        }

        let answer = list.chars().count();
        println!("{m}", m = list);
        answer as i32
    }
}

fn main() {
    assert_eq!(
        Solution::length_of_longest_substring("abcabcbb".to_string()),
        3
    );
}
