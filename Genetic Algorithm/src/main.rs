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
    let iterations = read_string(how_many_iterations);
    let num_of_candidates = 10;
    let xmin = -10;
    let xmax = 10;
    let population: Vec<i32> = random_population_generator(xmin, xmax, num_of_candidates);

    // Aptitud de la población.
    let population_results: Vec<f64> = evaluation(&population, &num_of_candidates);
    // Los mejores candidatos.
    let best_results = picking(&population_results);
    println!("Population: {:?}, \nPopulaton_results: {:?}, \nBest_results: {:?}", population, population_results, best_results);

}

fn picking(population_results: &Vec<f64>) -> HashMap<usize, f64> {
    let mut selection = HashMap::new();
    let number_of_selected = 3;
    let mut abs = make_absolute_vector(population_results);

    for _ in 0..number_of_selected {
        let abs_iter = abs.clone();
        let mut iterator = abs_iter.iter();
        let mut lowest = iterator.next().unwrap(); let mut place = 0;
        for i in 1..(abs.len() - 1) {
            //let next = iterator.next();
            let next = if let Some(thing) = iterator.next() { thing } else { break };

            if lowest > next { lowest = next; place = i; };
        }

        abs.remove(place);
        selection.insert(place, *lowest);
    }

    selection
}

fn make_absolute_vector(original: &Vec<f64>) -> Vec<f64> {
    let mut iterator = original.iter();
    let mut abs: Vec<f64> = Vec::new();

    for _ in 0..original.len() {
        if let Some(thing) = iterator.next() {
            abs.push(thing.abs());
        }
    }

    println!("Vector de resultados original: {:?} \nVector de resultados absoluto: {:?}", original, abs);
    abs
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