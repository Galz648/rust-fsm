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
        * panic/err handle on the println!("Raise error") line
        * add tests for the different cases of the controller
        * add p optimization
        * use an fsm to control (is this nicer to use ? ) - this is a good idea if there are many states to manage
*/

const HEAT_LOSS_PER_STEP: f32 = 0.5;
#[derive(Debug)]
struct Oven {
    current_temperature: f32,
    on: bool,
    power: f32,
}

impl Oven {
    fn perform_action(&mut self, action: &Option<Action>) {
        match action {
            Some(v) => {
                match v {
                    Action::HEAT(power) => {
                        self.heat(*power);
                    }
                    Action::NOTHING => {
                        self.on = false;
                        self.current_temperature -= HEAT_LOSS_PER_STEP; // TODO: move to process
                    }
                }
            }
            None => {
                println!("Raise error");
            } // TODO: unexpected functionality error
        }
    }

    fn heat(&mut self, power: f32) {
        self.on = true;
        self.power = power;
        self.current_temperature += self.power;
    }
}
#[derive(Debug)]
enum Action {
    // specific to the environment
    NOTHING,
    HEAT(f32),
}


fn main() {
    // e(t+h)- e(t) / h, h -> 0. This is the derivative of e(x) at x.

    println!("Nothing to see here!");
    let mut time_step = 0;
    let mut t = 0.0;
    let t_inc = 0.1;
    let mut action: Option<Action> = None;

    let mut oven = Oven {
        current_temperature: 20.0,
        on: false,
        power: 1.0,
    };
    let mut error: f32;
    const SETPOINT: f32 = 30.0;
    let controller = Controller {};
    loop {
        // increment the timestep
        time_step += 1;
        // translate time_step into time
        t = t_inc * time_step as f32; // wtf is this conversion

        // read the temperature from the sensor
        // calculate error
        error = calculate_error(SETPOINT, oven.current_temperature);

        // decide what action to take
        let action = controller.choose_action(error);

        // perform action
        oven.perform_action(&action);
        println!(
            "timestep: {:?} | action: {:?} | oven: {:?} |  error: {:?}",
            time_step, &action, oven, error
        );
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

fn calculate_error(setpoint: f32, current_temperature: f32) -> f32 {
    return setpoint - current_temperature;
}

fn process() {
    // adds noise to the controlled variable
}
struct Controller {}
impl Controller {
    fn choose_action(&self, error: f32) -> Option<Action> {
        if error > 0.0 {
            return Some(Action::HEAT(error))
        } else {
            return Some(Action::NOTHING);
        }
    }
}
