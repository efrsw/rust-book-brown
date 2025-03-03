enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter { state: UsState },
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter { state } => {
            println!("State quarter from {state:?}");
            25
        },
    }
}

fn plus_one(num: Option<i32>) -> Option<i32> {
    match num {
        Some(v) => Some(v + 1),
        None => None,
    }
}

fn testing_if_let() {
    let quarter = Some(Coin::Quarter { state: UsState::Alabama });
    if let Some(ref quarter_coin @ Coin::Quarter { state: ref alabama_state }) = quarter {
        println!("{:?}", alabama_state);

        if let Coin::Quarter { state } = quarter_coin {
            println!("{:?}", state);
        }
    }
}

fn main() {
    println!("The value is {}", value_in_cents(Coin::Quarter { state: UsState::Alabama } ));
}


