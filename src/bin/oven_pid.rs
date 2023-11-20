use std::{clone, f32::consts::E};

/*
    This is a simple program that will control the temperature of a room
    using a P controller. The program will read the temperature from a
    sensor and then turn on a heater if the temperature is too low. The
    program will also turn on a fan if the temperature is too high.

        |-desired temperature-|thermostat-error-controller -- plant(actuator, process)-|
        |--------------------------------------------------------------|
    The program will run in a loop and do the following:
        * Read the temperature from the sensor
        * Decide what action to take
        * Perform the action
        * Print the current state

    Should the controller be seperate from the oven?
    features to implement:
        * Create components and a controller
        * integrate the approximation of the derivative component
*/
use rand::Rng;
mod simulation;
use simulation::{Simulation, Updatable};
const HEAT_LOSS_PER_STEP: f32 = 0.5; // TODO: move to simulation

#[derive(Debug, Clone)]
struct Oven {
    current_temperature: f32,
    on: bool,
    power: f32,
}
impl Updatable for Oven {
    fn update(&mut self) {
        if self.on {
            let noise = rand::thread_rng().gen_range(-1.0..1.0); // generate noise
            self.current_temperature += self.power + noise;
        } else {
            self.current_temperature -= HEAT_LOSS_PER_STEP;
        }
    }
}

impl Oven {
    fn perform_action(&mut self, action: &Option<Action>) {
        match action {
            Some(v) => match v {
                Action::HEAT(power) => {
                    self.heat(*power);
                }
                Action::NOTHING => {
                    self.power = 0.0;
                    self.on = false;
                }
            },
            None => {
                println!("Raise error");
            } // TODO: unexpected functionality error
        }
    }

    fn heat(&mut self, power: f32) {
        self.on = true;
        self.power = power;
    }
}
#[derive(Debug)]
enum Action {
    // specific to the simulation
    NOTHING,
    HEAT(f32),
}
fn main() {
    // e(t+h)- e(t) / h, h -> 0. This is the derivative of e(x) at x.

    let mut sim = Simulation::new();

    // TODO: move to simulation

    // let mut time_step = 0;
    // let mut t = 0.0;
    // let t_inc = 0.1;

    // // TODO: move to simulation
    let mut oven = Oven {
        current_temperature: 20.0,
        on: false,
        power: 1.0,
    };
    let mut error: f32;
    const SETPOINT: f32 = 30.0;

    // intialize pid values
    let mut controller = Controller {
        d_states: Vec::new(),
        p_error: 0.0,
        d_error: 0.0,
        i_error: 0.0,
        k_p: 1.0,
        k_i: 1.0,
        k_d: 1.0,
    };

    controller.d_states.push(0.); // TODO: this is a hack to get the first derivative
                                  // add oven to simulation
                                  // sim.add_updatable(Box::new(oven.clone()));
                                  // sim.add_updatable(Box::new(controller.clone()));

    // run the simulation
    loop {
        // read the temperature from the sensor
        let current_temperature = oven.current_temperature;
        // calculate the error
        error = calculate_error(SETPOINT, current_temperature);
        // calculate the derivative of the error
        let d_error = controller.calculate_derivative_error(&error);
        // decide what action to take
        let action = controller.choose_action(&error);
        // log the state of the simulation components
        println!(
            "current_temperature: {:?}, error: {:?}, action: {:?} d_error: {:?}",
            current_temperature, error, action, d_error
        );
        // perform the action
        oven.perform_action(&action);
        // step the simulation
        //
        oven.update();
        // sim.step();
        std::thread::sleep(std::time::Duration::from_millis(500))
    }
}

fn calculate_error(setpoint: f32, current_temperature: f32) -> f32 {
    return setpoint - current_temperature;
}
#[derive(Debug, Clone)]
struct Controller {
    d_states: Vec<f32>,
    p_error: f32,
    d_error: f32,
    i_error: f32,
    k_p: f32,
    k_d: f32,
    k_i: f32,
}

impl Updatable for Controller {
    fn update(&mut self) {
        // update the controller
        // self.d_states.push(self.p_error);
        // self.p_error = 0.0;
        // self.d_error = 0.0;
        // self.i_error = 0.0;
    }
}
impl Controller {
    fn calculate_control_output(&self, error: &f32, d_error: &f32) -> f32 {
        let mut output: f32 = 0.0;
        output += self.k_p * error;
        // output += self.k_d * d_error;
        output
    }
    fn choose_action(&mut self, error: &f32) -> Option<Action> {
        // calculate the derivative of the error
        let d_error = &mut self.calculate_derivative_error(error);
        self.d_error = *d_error;
        let control_output = self.calculate_control_output(&error, &d_error);
        // decide the action to take based on the control output

        if control_output > 0.0 {
            return Some(Action::HEAT(control_output));
        } else {
            return Some(Action::NOTHING);
        }
    }

    fn last_state(&self) -> f32 {
        // return the last state
        match self.d_states.last() {
            //I know this to be true because It's initalized with one element
            Some(s) => {
                return *s;
            }
            None => {
                // raise error
                panic!("No state found")
            }
        }
    }

    fn calculate_derivative_error(&mut self, current_error: &f32) -> f32 {
        let error_derivative = current_error - self.last_state();
        error_derivative // No need to divide by time if it's always 1
    }
}

// unit tests
