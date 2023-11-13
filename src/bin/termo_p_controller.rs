/*
    This is a simple program that will control the temperature of a room
    using a P controller. The program will read the temperature from a
    sensor and then turn on a heater if the temperature is too low. The
    program will also turn on a fan if the temperature is too high.


    features to implement:
        * refactor the "perform action" to be an implementation of the Oven struct
        * panic/err handle on the println!("Raise error") line
        * add tests for the different cases of the controller
        * add proportional power usage (is this needed to become a "p controller"?)
        * use an fsm to control (is this nicer to use ? )
*/

#[derive(Debug)]

struct Oven {
    desired_temperature: i32,
    current_temperature: i32,
    on: bool,
    power: u32,
}

#[derive(Debug)]
enum Action {
    NOTHING,
    HEAT,
}
fn action_to_perform(current_temperature: i32, desired_temperature: i32) -> Action {
    if current_temperature > desired_temperature {
        return Action::NOTHING;
    } else {
        Action::HEAT
    }
}

fn perform_action(action: Option<Action>, oven: &mut Oven) -> &mut Oven {
    match action {
        Some(v) => {
            match v {
                Action::HEAT => {
                    // let error: i32 = oven.current_temperature - oven.desired_temperature; // should this be absolute?
                    oven.on = true;
                    oven.current_temperature += 1;
                    // return oven
                }
                Action::NOTHING => {
                    oven.on = false;
                    oven.current_temperature -= 1;
                }
            }
            oven
        }
        None => {
            println!("Raise error");
            return oven;
        } // TODO: unexpected functionality error
    }
}
fn main() {
    println!("Nothing to see here!");
    let mut time_step = 0;
    let mut action: Option<Action> = None;
    let mut oven = Oven {
        desired_temperature: 30,
        current_temperature: 20,
        on: false,
        power: 1,
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
        perform_action(action, &mut oven);

        println!("oven: {:?}", oven)
    }
}
