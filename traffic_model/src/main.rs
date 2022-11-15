use traffic_model::road::*;
//use std::time::Duration;

fn main() {
    let mut road: Road = Road::new();

    println!("{}\n\n\n", Road::generate_printing_road(&road));
    Road::mutate_road(&mut road );
    println!("{}", Road::generate_printing_road(&road));

}
