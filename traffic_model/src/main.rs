use traffic_model::road::*;

fn main() {
    let mut road: Road = Road::new();

    for cycle in 1..=100 {
        println!("Tiempo: {cycle}\n
            {}\n\n\n", Road::generate_printing_road(&road));
        Road::mutate_road(&mut road );
    }
}
