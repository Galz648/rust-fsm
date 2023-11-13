/*
    This is a simple program that will control the temperature of a room
    using a P controller. The program will read the temperature from a
    sensor and then turn on a heater if the temperature is too low. The
    program will also turn on a fan if the temperature is too high.


    features to implement:
        * refactor the "perform action" to be an implementation of the Oven struct
        * panic/err handle on the println!("Raise error") line
        * add tests for the different cases of the controller
        * add p optimization
        * use an fsm to control (is this nicer to use ? )
*/

const HEAT_LOSS_PER_STEP: f32 = 0.5;
#[derive(Debug)]
struct Oven {
    desired_temperature: f32,
    current_temperature: f32,
    on: bool,
    power: f32,
}

impl Oven {
    fn perform_action(&mut self, action: &Option<Action>) {
        match action {
            Some(v) => {
                match v {
                    Action::HEAT => {
                        let error = self.desired_temperature - self.current_temperature; // this is positive since the action is "HEAT"
                        self.power = error;
                        self.heat();
                    }
                    Action::NOTHING => {
                        self.on = false;
                        self.current_temperature -= HEAT_LOSS_PER_STEP;
                    }
                }
            }
            None => {
                println!("Raise error");
            } // TODO: unexpected functionality error
        }
    }

    fn heat(&mut self) {
        self.on = true;
        self.current_temperature += self.power;
    }
}
#[derive(Debug)]
enum Action {
    NOTHING,
    HEAT,
}
fn action_to_perform(current_temperature: f32, desired_temperature: f32) -> Action {
    if current_temperature >= desired_temperature {
        return Action::NOTHING;
    } else {
        Action::HEAT
    }
}

fn main() {
    println!("Nothing to see here!");
    let mut time_step = 0;
    let mut action: Option<Action> = None;
    let mut oven = Oven {
        desired_temperature: 30.0,
        current_temperature: 20.0,
        on: false,
        power: 1.0,
    };

    loop {
        // increment the timestep
        time_step += 1;
        // decide the action to perform
        action = Some(action_to_perform(
            oven.current_temperature,
            oven.desired_temperature,
        ));
        // perform action
        oven.perform_action(&action);

        println!(
            "timestep: {:?} | action: {:?} | oven: {:?}",
            time_step, &action, oven
        );
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
