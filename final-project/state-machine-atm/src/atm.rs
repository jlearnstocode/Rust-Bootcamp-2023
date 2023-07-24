//! The automated teller machine gives you cash after you swipe your card and enter your pin.
//! The atm may fail to give you cash if it is empty or you haven't swiped your card, or you have
//! entered the wrong pin.

use crate::traits::{hash, StateMachine};

/// The keys on the ATM keypad
#[derive(Copy, Clone, Debug, Hash)]
pub enum Key {
    One,
    Two,
    Three,
    Four,
    Enter,
}

/// Something you can do to the ATM
pub enum Action {
    /// Swipe your card at the ATM. The attached value is the hash of the pin
    /// that should be keyed in on the keypad next.
    SwipeCard(u64),
    /// Press a key on the keypad
    PressKey(Key),
}

/// The various states of authentication possible with the ATM
#[derive(PartialOrd, PartialEq, Debug, Clone)]
enum Auth {
    /// No session has begun yet. Waiting for the user to swipe their card
    Waiting,
    /// The user has swiped their card, providing the enclosed PIN hash.
    /// Waiting for the user to key in their pin
    Authenticating(u64),
    /// The user has authenticated. Waiting for them to key in the amount
    /// of cash to withdraw
    Authenticated,
}

/// The ATM. When a card is swiped, the ATM learns the correct pin's hash.
/// It waits for you to key in your pin. You can press as many numeric keys as
/// you like followed by enter. If the pin is incorrect, your card is returned
/// and the ATM automatically goes back to the main menu. If your pin is correct,
/// the ATM waits for you to key in an amount of money to withdraw. Withdraws
/// are bounded only by the cash in the machine (there is no account balance).
#[derive(Debug, Clone)]
pub struct Atm {
    /// How much money is in the ATM
    cash_inside: u64,
    /// The machine's authentication status.
    expected_pin_hash: Auth,
    /// All the keys that have been pressed since the last `Enter`
    keystroke_register: Vec<Key>,
}

//TODO
// Implement trait Default for Auth
// return Waiting status
impl Default for Auth {
    fn default() -> Self {
        return Auth::Waiting;
    }
}

//TODO
// Implement trait From  for &str
// Convert  elements in Key to &str
impl From<Key> for u64 {
    fn from(value: Key) -> Self {
        match value {
            Key::One => 1,
            Key::Two => 2,
            Key::Three => 3,
            Key::Four => 4,
            Key::Enter => 0,
        }
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        match (&self, other) {
            (Key::One, Key::One) => true,
            (Key::Two, Key::Two) => true,
            (Key::Three, Key::Three) => true,
            (Key::Four, Key::Four) => true,
            (Key::Enter, Key::Enter) => true,
            _ => false,
        }
    }
}

impl PartialEq for Atm {
    fn eq(&self, other: &Self) -> bool {
        self.cash_inside == other.cash_inside
            && self.expected_pin_hash == other.expected_pin_hash
            && self.keystroke_register == other.keystroke_register
    }
}
impl StateMachine for Atm {
    // Notice that we are using the same type for the state as we are using for the machine this time.
    type State = Atm;
    type Transition = Action;
    // Hint
    // Should use `default` method when auth status is Waiting status
    // Should use `from` method to convert  elements in Key to &str
    // Parse &str to integer to calculate amount
    // Use a hash function to verify the PIN both before and after the user presses the Enter key.
    fn next_state(starting_state: &Self::State, t: &Self::Transition) -> Self::State {
        // let key =
        // let new_keystoke_register = start_state.keystroke_register.clone();
        // new_keystoke_register.push(t::PressKey());

        let mut end_state = starting_state.clone();
        match t {
            Action::SwipeCard(num) => {
                if starting_state.expected_pin_hash == Auth::default() {
                    return Atm {
                        expected_pin_hash: Auth::Authenticating(*num),
                        keystroke_register: Vec::new(),
                        ..end_state
                    };
                } else {
                    return end_state;
                }
            }
            Action::PressKey(key) => {
                if starting_state.expected_pin_hash == Auth::Waiting {
                    end_state
                // } else if starting_state.expected_pin_hash == Auth::Authenticating(n) {
                //     end_state.keystroke_register.push(*key);
                //     end_state
                } else if starting_state.expected_pin_hash == Auth::Authenticated {
                    match key {
                        Key::Enter => {
                            let mut amount = 0;
                            let mut p: u32 = 0;
                            for value in starting_state.keystroke_register.iter().rev() {
                                let k_v: u64 = value.clone().into();
                                amount = amount + k_v * 10_u64.pow(p);
                                p = p + 1;
                            }

                            if amount > end_state.cash_inside {
                                return Atm {
                                    expected_pin_hash: Auth::Waiting,
                                    keystroke_register: vec![],
                                    ..end_state
                                };
                            } else {
                                return Atm {
                                    expected_pin_hash: Auth::Waiting,
                                    keystroke_register: vec![],
                                    cash_inside: end_state.cash_inside - amount,
                                };
                            }
                        }
                        _ => {
                            end_state.keystroke_register.push(key.clone());
                            end_state
                        }
                    }
                } else {
                    match key {
                        Key::Enter => {
                            let key_hash =
                                Auth::Authenticating(hash(&end_state.keystroke_register));
                            if key_hash == end_state.expected_pin_hash {
                                return Atm {
                                    expected_pin_hash: Auth::Authenticated,
                                    keystroke_register: vec![],
                                    ..end_state
                                };
                            } else {
                                return Atm {
                                    expected_pin_hash: Auth::Waiting,
                                    keystroke_register: vec![],
                                    ..end_state
                                };
                            }
                        }
                        _ => {
                            end_state.keystroke_register.push(key.clone());
                            end_state
                        }
                    }
                }
            }
        }
        // 1 Action -> SwipeCard -> expected_pin_hash 1234 PIN

        // 1234 exist -> SwipeCard -> No change
        // 1234 exist + keystroke_register exist -> No change
        // Authenticating (Not SwipeCard yet) -> PressKey -> no change
        // Auth::Authenticating(1234) ok ->PressKey-> add key
        // Auth::Authenticating(1234) + keystroke_register ->PressKey-> add key to end
        // Authenticating -> Enter -> sai  reset -> Auth Waiting
        // Authenticating -> Enter dung pass -> Authenticated
        // Authenticating -> PressKey(Key::Four) -> Get 4
        // Authenticating -> PressKey 14 withdraw 14 -> failse back to Waiting
    }
}

#[test]
fn sm_3_simple_swipe_card() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };
    let end = Atm::next_state(&start, &Action::SwipeCard(1234));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_swipe_card_again_part_way_through() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: Vec::new(),
    };
    let end = Atm::next_state(&start, &Action::SwipeCard(1234));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);

    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One, Key::Three],
    };
    let end = Atm::next_state(&start, &Action::SwipeCard(1234));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One, Key::Three],
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_press_key_before_card_swipe() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::One));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_enter_single_digit_of_pin() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: Vec::new(),
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::One));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One],
    };

    assert_eq!(end, expected);

    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One],
    };
    let end1 = Atm::next_state(&start, &Action::PressKey(Key::Two));
    let expected1 = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One, Key::Two],
    };

    assert_eq!(end1, expected1);
}

#[test]
fn sm_3_enter_wrong_pin() {
    // Create hash of pin
    let pin = vec![Key::One, Key::Two, Key::Three, Key::Four];
    let pin_hash = hash(&pin);

    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(pin_hash),
        keystroke_register: vec![Key::Three, Key::Three, Key::Three, Key::Three],
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::Enter));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_enter_correct_pin() {
    // Create hash of pin
    let pin = vec![Key::One, Key::Two, Key::Three, Key::Four];
    let pin_hash = hash(&pin);

    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(pin_hash),
        keystroke_register: vec![Key::One, Key::Two, Key::Three, Key::Four],
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::Enter));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_enter_single_digit_of_withdraw_amount() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: Vec::new(),
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::One));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: vec![Key::One],
    };

    assert_eq!(end, expected);

    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: vec![Key::One],
    };
    let end1 = Atm::next_state(&start, &Action::PressKey(Key::Four));
    let expected1 = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: vec![Key::One, Key::Four],
    };

    assert_eq!(end1, expected1);
}

#[test]
fn sm_3_try_to_withdraw_too_much() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: vec![Key::One, Key::Four],
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::Enter));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_withdraw_acceptable_amount() {
    let start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: vec![Key::One],
    };
    let end = Atm::next_state(&start, &Action::PressKey(Key::Enter));
    let expected = Atm {
        cash_inside: 9,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}
