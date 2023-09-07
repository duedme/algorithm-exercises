pub fn separate_strings(p: &str) -> String {

    let letters= p.chars();
    let mut indexes = Vec::new();

    for (i, l) in letters.enumerate() {
        if l.is_uppercase() { indexes.push(i) }
    }

    let mut j = 0_usize;
    let mut words: Vec<String> = Vec::new();

    for i in indexes {
        words.push(p[j..i].to_string());
        j = i;
    };
    words.push(p[j..].to_string());

    let words = words.iter()
        .map(|l| l.to_lowercase())
        .filter(|l| l != &"".to_string())
        .collect::<Vec<String>>();

    let words = words.join(" ");
    words

    /*
    for _ in &words { 
        //print!("{} ", word);
        io::stdout().flush().unwrap();
    }
    */
}

fn capitalize_multiple_words(input: &str) -> String {
    let words: Vec<&str> = input.split_whitespace().collect();

    if words.len() < 2 {
        return input.to_string();
    }

    let mut result = String::new();
    result.push_str(words[0]);

    for word in &words[1..] {
        let mut capitalized_word = word.to_string();
        if let Some(c) = capitalized_word.get_mut(0..1) {
            c.make_ascii_uppercase();
        }
        result.push_str(&capitalized_word);
    }

    result
}

pub fn basic_combine(p: &str, into: &str) -> String {
    let mut result = capitalize_multiple_words(p);

    match into.to_uppercase().as_str() {
        "V" => result,
        "C" => {
            if let Some(c) = result.get_mut(0..1) {
                c.make_ascii_uppercase();
            };
            result
        },
        "M" => {
            result.push_str("()");
            result
        },
        _ => panic!("Opciones equivocadas"),
    }
}