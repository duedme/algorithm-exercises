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