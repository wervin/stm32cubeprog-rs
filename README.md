# stm32cubeprog-rs 

This crate provides Rust bindings for the STM32CubeProgrammer API, facilitating the interaction with STMicroelectronics hardware programming tools.

## Compatibility

The crate is designed to be compatible with both Linux and Windows operating systems.

## Requirements

STM32CubeProgrammer version 2.14.0 or later must be installed on your system.

## Example Usage

The following example demonstrates how to discover connected STLinks on Linux, retrieve information, read and write memory, and program the attached device.

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate path to STM32CubeProgrammer folder
    let home_dir = std::env::var_os("HOME")
        .map(std::path::PathBuf::from)
        .expect("Failed to get home directory, $HOME variable missing");
    let binding = home_dir.join("Applications/STMicroelectronics/STM32Cube/STM32CubeProgrammer");
    let stm32prog_path = binding
        .to_str()
        .expect("Failed to join STM32CubeProgrammer path");
    
    // Load STM32CubeProgmmer API library
    let stm32prog = stm32cubeprog_rs::STM32CubeProg::new(stm32prog_path)?;

    // Find connected STLinks
    let mut stlinks = stm32prog.discover()?;
    for stlink in stlinks.iter_mut() {
        println!("{stlink}");
        
        // Configure the reset mode and the connection mode
        stlink.reset_mode(stm32cubeprog_rs::DebugResetMode::HardwareReset);
        stlink.connection_mode(stm32cubeprog_rs::DebugConnectMode::UnderReset);

        // Connect the STlink
        stm32prog.connect(stlink)?;
        
        // Fetch device information
        let device_info = stm32prog.device_info()?;
        println!("{device_info}");
        
        // Read and write register R0
        stm32prog.write_core_register(stm32cubeprog_rs::Register::R0, 0xAABBCCDD)?;
        let data = stm32prog.read_core_register(stm32cubeprog_rs::Register::R0)?;
        println!("R0:  0x{data:X}");
        
        // Read and write memory
        let data = stm32prog.read_memory8(0x1FFF7590, 16)?;
        println!("0x1FFF7590: {data:x?}");

        stm32prog.write_memory8(0x20000100, data)?;

        let data = stm32prog.read_memory32(0x1FFF7590, 4)?;
        println!("0x1FFF7590: {data:x?}");

        stm32prog.write_memory32(0x20000200, data)?;
        
        // Mass erase the device
        stm32prog.mass_erase()?;

        // Flash the device
        stm32prog.download("demo.hex", None, None, None)?;
        
        // Reset and disconnect the STLink
        stm32prog.reset(stlink)?;
        stm32prog.disconnect();
    }

    Ok(())
}
