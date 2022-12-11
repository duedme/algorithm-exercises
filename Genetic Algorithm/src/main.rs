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
    let population: Vec<f64> = random_population_generator(xmin, xmax, num_of_candidates);

    // Aptitud de la población.
    let population_results: Vec<f64> = evaluation(&population, &num_of_candidates);
    // Los mejores candidatos.
    let best_results = picking(&population_results);
    println!("Population: {:?}, \nPopulaton_results: {:?}, \nBest_results: {:?}", population, population_results, best_results);

    let merged_data = merge(&population, &best_results);

}

fn merge(popul: &Vec<f64>, refers: &HashMap<usize, f64>) -> Vec<f64> {
    let values = take(popul, refers);
    let mut return_value: Vec<f64> = Vec::new();
    println!("Values: {:?}", values);

    let mut i = 0;
    return_value.push(*values.get(i).unwrap());
    while values.get(i + 1).is_some() {
        return_value.push((values.get(i).unwrap() + values.get(i + 1).unwrap())/2.0);
        i += 1;
    };
    return_value.push(*values.get(i).unwrap());

    println!("Return_value: {:?}", return_value);
    return_value
}

fn take(popul: &Vec<f64>, refers: &HashMap<usize, f64>) -> Vec<f64> {
    let mut n_popul: Vec<f64> = Vec::new();
    let element = refers.iter();

    for (index, _) in element {
        n_popul.push(*popul.get(*index).unwrap());
    }

    n_popul
}

fn picking(population_results: &Vec<f64>) -> HashMap<usize, f64> {
    let mut selection = HashMap::new();
    let number_of_selected = 7;
    let mut abs = make_absolute_vector(population_results);

    for _ in 0..number_of_selected {
        let abs_iter = abs.clone();
        let mut iterator = abs_iter.iter();
        let mut lowest = iterator.next().unwrap(); let mut place = 0;
        for i in 1..(abs.len() - 1) {
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
fn evaluation(population: &Vec<f64>, num_of_candidates: &i32) -> Vec<f64> {
    let mut pop: Vec<f64> = Vec::new();

        for i in 0..*num_of_candidates {
            let x: f64 = *population.get(i as usize).unwrap() as f64;

            // Aquí se define la función           
            let function = x - 5 as f64;
            pop.push(function);
        };

        pop
}

fn read_string(message: String) -> i32 {
    println!("{}", message);

    let mut stdin_container = String::new();

    io::stdin().read_line(&mut stdin_container).expect("Problema al leer entrada.");
    let fixed = stdin_container.trim();

    let parsed = fixed.parse::<i32>();
    let return_val = match parsed {
        Ok(val) => val,
        Err(_) => panic!("Por favor entregue un númeor entero."),
    };

    return_val
}

fn random_population_generator(min: i32, max: i32, num_of_candidates: i32) -> Vec<f64> {
    let mut candidates: Vec<f64> = Vec::new();

    for _ in 0..num_of_candidates {
        let random_number = rand::thread_rng().gen_range(min..=max);
        candidates.push(random_number as f64);
    }

    println!("Los primeros candidatos son: {:?}", candidates);
    candidates
}