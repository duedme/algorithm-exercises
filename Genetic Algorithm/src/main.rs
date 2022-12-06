use std::io;
extern crate rand;
use rand::Rng;
use std::collections::HashMap;
// Generar población
// Evaluar Función Objectivo
// Criterio de término (si no, hay que hacer selección y cruzar individuos)
// Mejor Individuo

fn main() {
    let how_precise = String::from("¿Cuánto error soportará el algoritmo?");
    let how_many_iterations = String::from("¿Cuántas iteraciones deben haber?");
    let precision = read_string(how_precise);
    let interations = read_string(how_many_iterations);
    let num_of_candidates = 10;
    let xmin = -10;
    let xmax = 10;
    let population: Vec<i32> = random_population_generator(xmin, xmax, num_of_candidates);

    // Aptitud de la población.
    let population_results: Vec<f64> = evaluation(&population, &num_of_candidates);
    // Los mejores candidatos.
    let lowest_results = picking(&population_results);
    println!("Population: {:?}, \nPopulaton_results: {:?}, \nLowest_results: {:?}", population, population_results, lowest_results);

}

fn picking(population_results: &Vec<f64>) -> HashMap<usize, f64> {
    let mut selection = HashMap::new();
    let number_of_selected = 3;

    let mut i = 1000;
    let mut lowest = (population_results[0]).abs();
    let mut iterations = 0;

    while iterations != number_of_selected {

        for (index, number) in population_results.iter().enumerate() {
            if selection.get(&index).is_some() { continue; };
            if number.abs() < lowest.abs() { i = index; lowest = *number; };
            selection.insert(i, lowest);
        }
        iterations += 1;
    }

    selection
}

// Aquí va la función
fn evaluation(population: &Vec<i32>, num_of_candidates: &i32) -> Vec<f64> {
    let mut pop: Vec<f64> = Vec::new();

        for i in 0..*num_of_candidates {
            let x: f64 = *population.get(i as usize).unwrap() as f64;

            // Aquí se define la función           
            let function = x - 5 as f64;
            pop.push(function);
        };

        pop
}

fn read_string(message: String) -> String {
    println!("{}", message);

    let mut stdin_container = String::new();
    match io::stdin().read_line(&mut stdin_container) {
        Ok(_) => println!("Usted ha dicho: {}", stdin_container),
        Err(e) => println!("Ha habido un error al leer su dato: {}", e),
    };

    stdin_container
}

fn random_population_generator(min: i32, max: i32, num_of_candidates: i32) -> Vec<i32> {
    let mut candidates: Vec<i32> = Vec::new();

    for _ in 0..num_of_candidates {
        let random_number = rand::thread_rng().gen_range(min..=max);
        candidates.push(random_number);
    }

    println!("Los primeros candidatos son: {:?}", candidates);
    candidates
}