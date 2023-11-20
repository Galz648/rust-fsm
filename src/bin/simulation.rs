use std::fmt::Debug;

pub struct Simulation {
    updatables: Vec<Box<dyn Updatable>>,
    pub time_step: u32,
    pub state: Vec<Box<dyn Debug>>,
}

impl Simulation {
    pub fn new() -> Simulation {
        Simulation {
            updatables: Vec::new(),
            time_step: 0,
            state: Vec::new(),
        }
    }
    pub fn add_updatable(&mut self, updatable: Box<dyn Updatable>) {
        self.updatables.push(updatable);
    }


    // pub fn log(&self) -> String {
    //     // log the state of the simulation components
    //     return "log".to_string();
    // }
    pub fn step(&mut self) {
        //         // TODO: move to simulation

        // let mut time_step = 0;
        // let mut t = 0.0;
        // let t_inc = 0.1;

        // // TODO: move to simulation
        // let mut oven = Oven {
        //     current_temperature: 20.0,
        //     on: false,
        //     power: 1.0,
        // };

        // translate time_step into time
        // t = t_inc * time_step as f32; // wtf is this conversion

        //

        std::thread::sleep(std::time::Duration::from_millis(500)); // TODO: add parameter to control this
    }
}

pub trait Updatable {
    fn update(&mut self);
    // fn process(&mut self);
}
