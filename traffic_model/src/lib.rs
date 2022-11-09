pub mod car {
    pub struct Car {
        Velocity: i32,
        Position: i32,
    }

    impl Car {
        pub fn new_car(vel: i32, pos: i32) -> Car {
            Car {
                Velocity: vel,
                Position: pos,
            }
        }

        pub fn new_velocity(&self, vel_max: i32, probability: i32) -> i32 {
            use rng::Rng;

            let mut range = rand::thread_rng();

        }
    }
}