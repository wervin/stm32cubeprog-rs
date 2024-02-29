fn main() -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = std::env::var_os("HOME")
        .map(std::path::PathBuf::from)
        .expect("Failed to get home directory, $HOME variable missing");
    let binding = home_dir.join("Applications/STMicroelectronics/STM32Cube/STM32CubeProgrammer");
    let stm32prog_path = binding
        .to_str()
        .expect("Failed to join STM32CubeProgrammer path");
    let stm32prog = stm32cubeprog_rs::STM32CubeProg::new(stm32prog_path)?;
    let stlinks = stm32prog.discover()?;
    stlinks.iter().for_each(|stlink| {
        println!("{stlink}");
        // stm32prog.connect(stlink);
    });

    Ok(())
}
