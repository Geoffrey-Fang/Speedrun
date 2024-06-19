use std::error::Error;

use Speedrun::stock::info::cls::{clsTelegraph, handleClsTelegraph};

fn main() -> Result<(), Box<dyn Error>> {
    match clsTelegraph() {
        Ok((r, c, l)) => handleClsTelegraph(r, c, l),
        Err(_) => todo!(),
    }
    Ok(())
}