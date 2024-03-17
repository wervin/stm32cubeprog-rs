fn main() -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = std::env::var_os("HOME")
        .map(std::path::PathBuf::from)
        .expect("Failed to get home directory, $HOME variable missing");
    let binding = home_dir.join("Applications/STMicroelectronics/STM32Cube/STM32CubeProgrammer");
    let stm32prog_path = binding
        .to_str()
        .expect("Failed to join STM32CubeProgrammer path");
    let stm32prog = stm32cubeprog_rs::STM32CubeProg::new(stm32prog_path)?;
    let mut stlinks = stm32prog.discover()?;
    for stlink in stlinks.iter_mut() {
        println!("{stlink}");
        stlink.reset_mode(stm32cubeprog_rs::DebugResetMode::HardwareReset);
        stlink.connection_mode(stm32cubeprog_rs::DebugConnectMode::UnderReset);
        stm32prog.connect(stlink)?;

        let device_info = stm32prog.device_info()?;
        println!("{device_info}");

        stm32prog.write_core_register(stm32cubeprog_rs::Register::R0, 0xAABBCCDD)?;
        let data = stm32prog.read_core_register(stm32cubeprog_rs::Register::R0)?;
        println!("data 0x{data:X}");

        // stm32prog.mass_erase()?;
        // stm32prog.download("tests/demo.hex", None, None, None)?;
        stm32prog.reset(stlink)?;
        stm32prog.disconnect();
    }

    Ok(())
}
