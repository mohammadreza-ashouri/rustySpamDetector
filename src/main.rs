
extern crate rustySpamDetector;
use rustySpamDetector::corelib;

fn main() -> Result<(), std::io::Error> {

    let mut user_message = "eat our BS product to lose your weight up to 17%.";
    let mut score_associated = corelib::score(user_message)?;
    let mut state = corelib::identify(user_message)?;
    println!("{:.4?}", user_message);
    println!("{:?}", state);


    user_message = "join to our discord channel for receiving the latest signal for crypto pump and dump...";
    score_associated= corelib::score(user_message)?;
    state = corelib::identify(user_message)?;
    println!("{:.4?}", score_associated);
    println!("{:?}", state);



    user_message = "Hey Mo, can you send me your solutions for the Solana contract?";
    score_associated= corelib::score(user_message)?;
    state = corelib::identify(user_message)?;
    println!("{:.4?}", score_associated);
    println!("{:?}", state);


    Ok(())
}
