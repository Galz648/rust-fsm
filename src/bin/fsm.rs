/*
fn state_transition (start_state: State, end_state: State) {
}

// valid transitions:

Green -> Yellow
Yellow -> Red
Red -> Yellow
Yellow -> Green


// invalid transitions:

Green -> Red
*/

impl Fsm {
    fn is_valid_transition(&self, transition: &Transition) -> bool {
        self.valid_transitions.iter().any(|valid_transition| {
            transition.start_state == valid_transition.start_state
                && transition.end_state == valid_transition.end_state
        })
    }
}

#[derive(PartialEq)]
struct Transition {
    start_state: State,
    end_state: State,
}

struct Fsm {
    // valid transitions
    valid_transitions: Vec<Transition>,
}

#[derive(PartialEq)]
enum State {
    Red,
    Green,
    Yellow,
}

fn main() {
    let invalid_transition = Transition {
        start_state: State::Green,
        end_state: State::Green,
    };

    let transitions = vec![
        Transition {
            start_state: State::Green,
            end_state: State::Yellow,
        },
        Transition {
            start_state: State::Yellow,
            end_state: State::Red,
        },
        Transition {
            start_state: State::Yellow,
            end_state: State::Green,
        },
        Transition {
            start_state: State::Red,
            end_state: State::Yellow,
        },
    ];
    let fsm: Fsm = Fsm {
        valid_transitions: transitions,
    }; // I wonder why there is a semi-colon here since it returns a value, an initialized Fsm struct

    let is_valid = fsm.is_valid_transition(&invalid_transition);

    println!("transition validity: {}", is_valid)
}
