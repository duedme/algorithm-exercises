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
            if (velocity + 1 <= max_speed) && (velocity + 1 <= distance) 
            { velocity + 1 } else { distance }
        }

        pub fn update_position(&self, max_speed: u8, distance_between_cars: u8) -> (u8, u8) {
            let mut velocity = Car::calc_new_velocity_with_max_speed_and_no_crash(
                self.velocity, max_speed, distance_between_cars
            );

            velocity = Car::calc_maybe_slow_down(velocity);
        
            let position = 200 % (self.position + velocity);

            (velocity, position)
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

            for i in 0..space.len() {
                match space[i] {
                    None => continue,
                    Some(mut car) => {
                        /*let mut distance = 0;
                        let mut j = i + 1;

                        while space[j] != None {
                            distance += 1;
                            j += 1;
                        }
                        */

                        let mut j = i + 1;
                        println!("j + 1 al principio: {j}");
                        while space[j] == None { j += 1;
                            j = 200 % j;
                            println!("j en proceso: {j}"); 
                        };

                        let distance = space[j].unwrap().position - car.position - 1;
                        println!("Distancia al siguiente es: {distance}");

                        (car.velocity, car.position) = car.update_position(self.max_speed, distance);
                        space[i] = None;
                        space[car.position as usize] = Some(car);
                    }
                }
            }

//            println!("El space nuevo es: \n\n{:?}", space);

            self.road = space;
        }

    }
}
