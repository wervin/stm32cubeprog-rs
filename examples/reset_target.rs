extern crate stm32cubeprog_rs;

use std::env;
use std::error::Error;

// TODO: Fix slice from raw parts when no ST-Link is connected
fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let stm32prog_path = env::var("CUBE_API_DIR")?;

    // Load STM32CubeProgmmer API library
    let stm32prog = stm32cubeprog_rs::STM32CubeProg::new(stm32prog_path)?;

    // Find connected STLinks
    let mut stlinks = stm32prog.discover()?;

    for stlink in stlinks.iter_mut() {
        println!("{stlink}");

        // Connect the STlink
        stm32prog.connect(stlink)?;

        // Reset and disconnect the STLink
        stm32prog.reset(stlink)?;
        stm32prog.disconnect();
    }

    Ok(())
}
