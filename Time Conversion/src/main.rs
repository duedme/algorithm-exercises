<<<<<<< HEAD
fn mini_max_sum(arr: &[i32]) {
    
    let (mini_pos, maxi_pos) = {

        let mut sorted = arr.to_owned();
        sorted.sort();

        let mini = sorted[0];
        let maxi = sorted[4];

        let mini_pos = arr.iter().position(|x| x == &mini ).unwrap();
        let maxi_pos = arr.iter().position(|x| x == &maxi).unwrap();
        
        (mini_pos, maxi_pos)
    };

    println!("Posiciones: mini: {}, maxi: {}", mini_pos, maxi_pos);

    let mut suma_mini: i64 = 0;
    let mut suma_maxi: i64 = 0;
    
    for (pos, item) in arr.iter().enumerate() {
        if pos != mini_pos { suma_mini += *item as i64 }
        if pos != maxi_pos { suma_maxi += *item as i64}
    }

    println!("{} {}", suma_maxi, suma_mini);
}

fn main() {
    mini_max_sum(&[256741038, 623958417, 467905213, 714532089, 938071625]);
}
=======
fn time_conversion(s: &str) -> String {

    let st = match s.strip_suffix("AM") {
        Some(ns) => {

            let mut ns = ns.to_string();
            
            fn form(n: &str) -> String {
                let n = n.parse::<u8>().expect(format!("PRoblema al convertir: {}", n).as_str());

                if n < 10 {
                    return format!("0{}", n)
                }

                return n.to_string()
            }

            if &s[0..=1] == "12" {
                ns = format!("{}:{}:{}", "00", form(&s[3..=4]), form(&s[6..=7]));
            } else {
                ns = format!("{}:{}:{}", s[0..=1].to_string(), form(&s[3..=4]), form(&s[6..=7]));
            }
            ns.to_string()
        },
        None => {

            fn form(n: &u8) -> String {
                if n < &10 {
                    return format!("0{}", n)
                }

                return n.to_string()
            }

            let mut hh = s[0..=1].parse::<u8>().expect("Error in hour");
            hh = if hh < 12 { hh + 12 } else { hh };
            let mm = &s[3..=4].parse::<u8>().expect("Error in minutes");
            let ss = &s[6..=7].parse::<u8>().expect("Error in seconds");

            format!("{hh}:{mm}:{ss}", hh = form(&hh), mm = form(&mm), ss = form(&ss))
        }
    };

    st
}

fn main() {
    println!("{}", time_conversion("07:42:22AM"));
}
>>>>>>> 45e1c89 (Ejercicios 2 y 3.)
