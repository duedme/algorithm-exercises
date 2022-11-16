pub mod car {

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Car {
        pub velocity: u8,
        pub position: u8,
    }

    impl Car {
        pub fn new(vel: u8, pos: u8) -> Car {
            Car {
                velocity: vel,
                position: pos,
            }
        }

        fn calc_maybe_slow_down(velocity: u8) -> u8{
            let probability_of_slowing_down = rand::random::<f64>();
            println!("La probalididad de slow es: {probability_of_slowing_down}");
            let mut new_velocity = velocity.clone();

            if probability_of_slowing_down > 0.5 && velocity > 0 {
                new_velocity = velocity - 1
            };

            new_velocity
        }

        fn calc_new_velocity_with_max_speed_and_no_crash(velocity: u8, max_speed: u8, distance: u8) -> u8 {
            println!("Velocidad antes de max_speed no_crash es: {velocity} \n\t
                recordando que la distancia al siguiente es: {distance}");
            if (velocity + 1 <= max_speed) && (velocity + 1 <= distance) 
            { velocity + 1 } else { distance }
        }

        pub fn update_position(&self, max_speed: u8, distance_between_cars: u8) -> (u8, u8) {
            let mut velocity = Car::calc_new_velocity_with_max_speed_and_no_crash(
                self.velocity, max_speed, distance_between_cars
            );

            println!("La velocidad antes de disminuir es: {velocity}");
            velocity = Car::calc_maybe_slow_down(velocity);
            println!("La velocidad final es: {velocity}");
        
            let aux_pos = self.position as i32 + velocity as i32;
            let position = if aux_pos >= 200 { aux_pos - 200 } else { aux_pos };
            println!("La posición final es: {position}");

            (velocity, position as u8)
        }
    }
}

pub mod road {

    use crate::car::Car;

    pub struct Road {
        pub road:[Option<Car>; 200],
        pub max_speed: u8,
    }

    impl Road{
        fn generate_empty_road() -> Road {
            let road = Road {
                road: [None; 200],
                max_speed: 5,
            };

            road
        }

        pub fn new() -> Road {
            use rand::*;

            let mut track = Road::generate_empty_road();
            let number_of_cars: u8 = 66;

            for _ in 0..=number_of_cars {
                let mut rng = rand::thread_rng();
                let place_rand: usize = rng.gen_range(0..200);
                let velocity_rand = rng.gen_range(0..=5);
                
                track.road[place_rand] = Some(Car::new(velocity_rand, place_rand as u8));
            }

            track
        }

        pub fn generate_printing_road(&self) -> String {
            let mut returning_road = String::new();

            for space in &self.road {
                returning_road.push_str("|| ");
                
                match space {
                    None => returning_road.push_str("_"),
                    Some(car) => returning_road.push_str(&(car.velocity.to_string())),
                }

                returning_road.push_str(" ");
            }

            returning_road.push_str("||");

            returning_road
        }

        // Here will be the parameters for changing the road based on the Car object.
        pub fn mutate_road(&mut self) {
            let mut space: [Option<Car>; 200] = [None; 200];

            //Creates a clone 'space' of Road.road.
            for index in 0..200 {
                space[index] = self.road[index].clone();
            }

            for i in 0..self.road.len() {
                match space[i] {
                    None => continue,
                    Some(mut car) => {

                        // Buscando distancia
                        // Qué pasa si la posición de adelante es 2 y la de atrás es 199 o 198?
                        let mut j = i + 1;
                        println!("j + 1 al principio: {j}");
                        loop { 
                            j += 1;
                            if j == 201 { j = 1 };
                            if j == i + 1 { break; }
                            println!("j en proceso: {j}"); 
                            if self.road[j - 1] != None
                            { println!("{:?}", self.road[j-1]);break; };
                        };

                        // Obteniendo la distancia restando las posiciones.
                        let distance: i32;

                        if self.road[j - 1].unwrap().position > 200 { 
                            distance = (self.road[j - 1].unwrap().position as i32 + 200) - (car.position as i32) - 1;
                        } else {
                            distance = (self.road[j - 1].unwrap().position as i32) - (car.position as i32) - 1;
                        };
                        let distance = distance as u8;
                        println!("Distancia al siguiente es: {distance}");

                        (car.velocity, car.position) = car.update_position(self.max_speed, distance);
                        space[i] = None;
                        println!("Car.velocity es: {}", car.velocity);
                        println!("Car.position es: {}", car.position);
                        space[car.position as usize] = Some(car);
                    }
                }
            }

//            println!("El space nuevo es: \n\n{:?}", space);
            assert_ne!(self.road, space);
            self.road = space;
        }

    }
}
