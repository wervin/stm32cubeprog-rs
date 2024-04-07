//!
//! Rust bindings for the STM32CubeProgrammer API library.
//!
//! # Compatibility
//!
//! This crate is compatible with both Linux and Windows.
//!
//! # Requirements
//! This crate requires STM32CubeProgrammer (version 2.14.0 or later) to be installed.
//!
//! # Example
//!
//! The code below will discover connected STLinks on Linux, retrieve information, read and write memory, and finally program the attached device.
//!
//! ```
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Generate path to STM32CubeProgrammer folder
//!     let home_dir = std::env::var_os("HOME")
//!         .map(std::path::PathBuf::from)
//!         .expect("Failed to get home directory, $HOME variable missing");
//!     let binding = home_dir.join("Applications/STMicroelectronics/STM32Cube/STM32CubeProgrammer");
//!     let stm32prog_path = binding
//!         .to_str()
//!         .expect("Failed to join STM32CubeProgrammer path");
//!     
//!     // Load STM32CubeProgmmer API library
//!     let stm32prog = stm32cubeprog_rs::STM32CubeProg::new(stm32prog_path)?;
//!
//!     // Find connected STLinks
//!     let mut stlinks = stm32prog.discover()?;
//!     for stlink in stlinks.iter_mut() {
//!         println!("{stlink}");
//!         
//!         // Configure the reset mode and the connection mode
//!         stlink.reset_mode(stm32cubeprog_rs::DebugResetMode::HardwareReset);
//!         stlink.connection_mode(stm32cubeprog_rs::DebugConnectMode::UnderReset);
//!
//!         // Connect the STlink
//!         stm32prog.connect(stlink)?;
//!         
//!         // Fetch device information
//!         let device_info = stm32prog.device_info()?;
//!         println!("{device_info}");
//!         
//!         // Read and write register R0
//!         stm32prog.write_core_register(stm32cubeprog_rs::Register::R0, 0xAABBCCDD)?;
//!         let data = stm32prog.read_core_register(stm32cubeprog_rs::Register::R0)?;
//!         println!("R0:  0x{data:X}");
//!         
//!         // Read and write memory
//!         let data = stm32prog.read_memory8(0x1FFF7590, 16)?;
//!         println!("0x1FFF7590: {data:x?}");
//!
//!         stm32prog.write_memory8(0x20000100, data)?;
//!
//!         let data = stm32prog.read_memory32(0x1FFF7590, 4)?;
//!         println!("0x1FFF7590: {data:x?}");
//!
//!         stm32prog.write_memory32(0x20000200, data)?;
//!         
//!         // Mass erase the device
//!         stm32prog.mass_erase()?;
//!
//!         // Flash the device
//!         stm32prog.download("demo.hex", None, None, None)?;
//!         
//!         // Reset and disconnect the STLink
//!         stm32prog.reset(stlink)?;
//!         stm32prog.disconnect();
//!     }
//!
//!     Ok(())
//! }
//! ```

pub mod err;

#[cfg(unix)]
#[allow(non_camel_case_types)]
pub type wchar = u32;
#[cfg(windows)]
#[allow(non_camel_case_types)]
pub type wchar = u16;

#[repr(C)]
pub struct DisplayCallbacks {
    pub init_progress_bar: extern "C" fn(),
    pub log_message: extern "C" fn(msg_type: std::os::raw::c_int, msg: *const std::os::raw::c_int),
    pub load_bar: extern "C" fn(current: std::os::raw::c_int, total: std::os::raw::c_int),
}

extern "C" fn init_progress_bar() {}

extern "C" fn log_message(_msg_type: std::os::raw::c_int, _msg: *const std::os::raw::c_int) {}

extern "C" fn load_bar(_current: std::os::raw::c_int, _total: std::os::raw::c_int) {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum Verbosity {
    Level0 = 0,
    Level1 = 1,
    Level2 = 2,
    Level3 = 3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum DebugPort {
    Jtag = 0,
    Swd = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum DebugConnectMode {
    Normal = 0,
    HotPlug = 1,
    UnderReset = 2,
    PowerDown = 3,
    PreReset = 4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum DebugResetMode {
    SoftwareReset = 0,
    HardwareReset = 1,
    CoreReset = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Frequencies {
    pub jtag_freq: [std::os::raw::c_uint; 12usize],
    pub jtag_freq_count: std::os::raw::c_uint,
    pub swd_freq: [std::os::raw::c_uint; 12usize],
    pub swd_freq_count: std::os::raw::c_uint,
}

impl Frequencies {
    pub fn jtag_frequencies(&self) -> Vec<u32> {
        let size = std::convert::TryInto::try_into(self.jtag_freq_count).unwrap_or_default();
        self.jtag_freq.to_vec()[0..size].to_vec()
    }

    pub fn swd_frequencies(&self) -> Vec<u32> {
        let size = std::convert::TryInto::try_into(self.swd_freq_count).unwrap_or_default();
        self.swd_freq.to_vec()[0..size].to_vec()
    }
}

impl std::fmt::Display for Frequencies {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "\
JTAG Frequencies Available: {:?},
SWD Frequencies Available: {:?}",
            self.jtag_frequencies(),
            self.swd_frequencies(),
        )
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DeviceGeneralInfo {
    pub device_id: ::std::os::raw::c_ushort,
    pub flash_size: ::std::os::raw::c_int,
    pub bootloader_version: ::std::os::raw::c_int,
    pub category: [::std::os::raw::c_char; 4usize],
    pub cpu: [::std::os::raw::c_char; 20usize],
    pub name: [::std::os::raw::c_char; 100usize],
    pub series: [::std::os::raw::c_char; 100usize],
    pub description: [::std::os::raw::c_char; 150usize],
    pub revision_id: [::std::os::raw::c_char; 8usize],
    pub board: [::std::os::raw::c_char; 100usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DebugConnectParameters {
    pub debug_port: DebugPort,
    pub index: std::os::raw::c_int,
    pub serial_number: [std::os::raw::c_char; 33usize],
    pub firmware_version: [std::os::raw::c_char; 20usize],
    pub target_voltage: [std::os::raw::c_char; 5usize],
    pub access_port_count: std::os::raw::c_int,
    pub access_port: std::os::raw::c_int,
    pub connection_mode: DebugConnectMode,
    pub reset_mode: DebugResetMode,
    pub old_firmware: std::os::raw::c_int,
    pub frequencies: Frequencies,
    pub frequency: std::os::raw::c_int,
    pub bridge: std::os::raw::c_int,
    pub shared: std::os::raw::c_int,
    pub board: [std::os::raw::c_char; 100usize],
    pub debug_sleep: std::os::raw::c_int,
    pub speed: std::os::raw::c_int,
}

type SetLoaderPath = unsafe extern "C" fn(path: *const std::os::raw::c_char);
type SetDisplayCallbacks = unsafe extern "C" fn(c: DisplayCallbacks);
type SetVerbosityLevel = unsafe extern "C" fn(level: Verbosity);
type GetStLinkList = unsafe extern "C" fn(
    debug_connect_parameters: *mut *mut DebugConnectParameters,
    shared: std::os::raw::c_int,
) -> std::os::raw::c_int;
type ConnectStLink =
    unsafe extern "C" fn(debug_connect_parameters: DebugConnectParameters) -> std::os::raw::c_int;
type Disconnect = unsafe extern "C" fn();
type Reset = unsafe extern "C" fn(reset_mode: DebugResetMode) -> std::os::raw::c_int;
type MassErase = unsafe extern "C" fn() -> std::os::raw::c_int;
type DownloadFile = unsafe extern "C" fn(
    file_path: *const wchar,
    address: std::os::raw::c_uint,
    skip_erase: std::os::raw::c_uint,
    verify: std::os::raw::c_uint,
    path: *const wchar,
) -> std::os::raw::c_int;
type GetDeviceGeneralInfo = unsafe extern "C" fn() -> *mut DeviceGeneralInfo;
type ReadMemory = unsafe extern "C" fn(
    address: ::std::os::raw::c_uint,
    data: *mut *mut ::std::os::raw::c_uchar,
    size: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int;
type WriteMemory = unsafe extern "C" fn(
    address: ::std::os::raw::c_uint,
    data: *mut ::std::os::raw::c_uchar,
    size: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int;
type ReadCoreRegister = unsafe extern "C" fn(
    register: std::os::raw::c_uint,
    data: *mut std::os::raw::c_uint,
) -> ::std::os::raw::c_int;
type WriteCoreRegister = unsafe extern "C" fn(
    register: std::os::raw::c_uint,
    data: std::os::raw::c_uint,
) -> ::std::os::raw::c_int;

#[cfg(unix)]
pub struct VTable {
    set_loaders_path: libloading::os::unix::Symbol<SetLoaderPath>,
    set_display_callbacks: libloading::os::unix::Symbol<SetDisplayCallbacks>,
    set_verbosity_level: libloading::os::unix::Symbol<SetVerbosityLevel>,
    get_stlink_list: libloading::os::unix::Symbol<GetStLinkList>,
    connect_stlink: libloading::os::unix::Symbol<ConnectStLink>,
    disconnect: libloading::os::unix::Symbol<Disconnect>,
    reset: libloading::os::unix::Symbol<Reset>,
    mass_erase: libloading::os::unix::Symbol<MassErase>,
    download_file: libloading::os::unix::Symbol<DownloadFile>,
    get_device_general_info: libloading::os::unix::Symbol<GetDeviceGeneralInfo>,
    read_memory: libloading::os::unix::Symbol<ReadMemory>,
    write_memory: libloading::os::unix::Symbol<WriteMemory>,
    read_core_register: libloading::os::unix::Symbol<ReadCoreRegister>,
    write_core_register: libloading::os::unix::Symbol<WriteCoreRegister>,
}

#[cfg(windows)]
pub struct VTable {
    set_loaders_path: libloading::os::windows::Symbol<SetLoaderPath>,
    set_display_callbacks: libloading::os::windows::Symbol<SetDisplayCallbacks>,
    set_verbosity_level: libloading::os::windows::Symbol<SetVerbosityLevel>,
    get_stlink_list: libloading::os::windows::Symbol<GetStLinkList>,
    connect_stlink: libloading::os::windows::Symbol<ConnectStLink>,
    disconnect: libloading::os::windows::Symbol<Disconnect>,
    reset: libloading::os::windows::Symbol<Reset>,
    mass_erase: libloading::os::windows::Symbol<MassErase>,
    download_file: libloading::os::windows::Symbol<DownloadFile>,
    get_device_general_info: libloading::os::windows::Symbol<GetDeviceGeneralInfo>,
    read_memory: libloading::os::windows::Symbol<ReadMemory>,
    write_memory: libloading::os::windows::Symbol<WriteMemory>,
    read_core_register: libloading::os::windows::Symbol<ReadCoreRegister>,
    write_core_register: libloading::os::windows::Symbol<WriteCoreRegister>,
}

impl VTable {
    fn new(library: &libloading::Library) -> Result<Self, err::Error> {
        let set_loaders_path: libloading::Symbol<SetLoaderPath> =
            unsafe { library.get(b"setLoadersPath\0")? };
        let set_loaders_path = unsafe { set_loaders_path.into_raw() };
        let set_display_callbacks: libloading::Symbol<SetDisplayCallbacks> =
            unsafe { library.get(b"setDisplayCallbacks\0")? };
        let set_display_callbacks = unsafe { set_display_callbacks.into_raw() };
        let set_verbosity_level: libloading::Symbol<SetVerbosityLevel> =
            unsafe { library.get(b"setVerbosityLevel\0")? };
        let set_verbosity_level = unsafe { set_verbosity_level.into_raw() };
        let get_stlink_list: libloading::Symbol<GetStLinkList> =
            unsafe { library.get(b"getStLinkList\0")? };
        let get_stlink_list = unsafe { get_stlink_list.into_raw() };
        let connect_stlink: libloading::Symbol<ConnectStLink> =
            unsafe { library.get(b"connectStLink\0")? };
        let connect_stlink = unsafe { connect_stlink.into_raw() };
        let disconnect: libloading::Symbol<Disconnect> = unsafe { library.get(b"disconnect\0")? };
        let disconnect = unsafe { disconnect.into_raw() };
        let reset: libloading::Symbol<Reset> = unsafe { library.get(b"reset\0")? };
        let reset = unsafe { reset.into_raw() };
        let mass_erase: libloading::Symbol<MassErase> = unsafe { library.get(b"massErase\0")? };
        let mass_erase = unsafe { mass_erase.into_raw() };
        let download_file: libloading::Symbol<DownloadFile> =
            unsafe { library.get(b"downloadFile\0")? };
        let download_file = unsafe { download_file.into_raw() };
        let get_device_general_info: libloading::Symbol<GetDeviceGeneralInfo> =
            unsafe { library.get(b"getDeviceGeneralInf\0")? };
        let get_device_general_info = unsafe { get_device_general_info.into_raw() };
        let read_memory: libloading::Symbol<ReadMemory> = unsafe { library.get(b"readMemory\0")? };
        let read_memory = unsafe { read_memory.into_raw() };
        let write_memory: libloading::Symbol<WriteMemory> =
            unsafe { library.get(b"writeMemory\0")? };
        let write_memory = unsafe { write_memory.into_raw() };
        let read_core_register: libloading::Symbol<ReadCoreRegister> =
            unsafe { library.get(b"readCortexReg\0")? };
        let read_core_register = unsafe { read_core_register.into_raw() };
        let write_core_register: libloading::Symbol<WriteCoreRegister> =
            unsafe { library.get(b"writeCortexRegistres\0")? };
        let write_core_register = unsafe { write_core_register.into_raw() };

        Ok(VTable {
            set_loaders_path,
            set_display_callbacks,
            set_verbosity_level,
            get_stlink_list,
            connect_stlink,
            disconnect,
            reset,
            mass_erase,
            download_file,
            get_device_general_info,
            read_memory,
            write_memory,
            read_core_register,
            write_core_register,
        })
    }
}

#[derive(Debug, Clone)]
pub struct STLink {
    debug_connect_parameters: DebugConnectParameters,
}

impl STLink {
    pub fn frequencies(&self) -> Frequencies {
        self.debug_connect_parameters.frequencies
    }

    pub fn debug_port(&self) -> DebugPort {
        self.debug_connect_parameters.debug_port
    }

    pub fn index(&self) -> i32 {
        self.debug_connect_parameters.index
    }

    pub fn access_port_count(&self) -> i32 {
        self.debug_connect_parameters.access_port_count
    }

    pub fn access_port(&self) -> i32 {
        self.debug_connect_parameters.access_port
    }

    pub fn connection_mode(&self) -> DebugConnectMode {
        self.debug_connect_parameters.connection_mode
    }

    pub fn reset_mode(&self) -> DebugResetMode {
        self.debug_connect_parameters.reset_mode
    }

    pub fn old_firmware(&self) -> bool {
        self.debug_connect_parameters.old_firmware == 1
    }

    pub fn frequency(&self) -> i32 {
        self.debug_connect_parameters.frequency
    }

    pub fn bridge(&self) -> bool {
        self.debug_connect_parameters.bridge == 1
    }

    pub fn shared(&self) -> bool {
        self.debug_connect_parameters.shared == 1
    }

    pub fn debug_sleep(&self) -> bool {
        self.debug_connect_parameters.debug_sleep == 1
    }

    pub fn speed(&self) -> i32 {
        self.debug_connect_parameters.speed
    }

    pub fn target_voltage(&self) -> Result<f32, err::Error> {
        Ok(String::from_utf8(
            self.debug_connect_parameters
                .target_voltage
                .iter()
                .map(|&c| c as u8)
                .collect(),
        )?.trim_matches(char::from(0)).parse()?)
    }

    pub fn serial_number(&self) -> Result<String, err::Error> {
        Ok(String::from_utf8(
            self.debug_connect_parameters
                .serial_number
                .iter()
                .map(|&c| c as u8)
                .collect(),
        )?.trim_matches(char::from(0)).to_owned())
    }

    pub fn firmware_version(&self) -> Result<String, err::Error> {
        Ok(String::from_utf8(
            self.debug_connect_parameters
                .firmware_version
                .iter()
                .map(|&c| c as u8)
                .collect(),
        )?.trim_matches(char::from(0)).to_owned())
    }

    pub fn board(&self) -> Result<String, err::Error> {
        Ok(String::from_utf8(
            self.debug_connect_parameters
                .board
                .iter()
                .map(|&c| c as u8)
                .collect(),
        )?.trim_matches(char::from(0)).to_owned())
    }

    pub fn set_access_port(&mut self, access_port: i32) {
        self.debug_connect_parameters.access_port = access_port;
    }

    pub fn set_frequency(&mut self, frequency: i32) {
        self.debug_connect_parameters.frequency = frequency;
    }

    pub fn set_reset_mode(&mut self, reset_mode: DebugResetMode) {
        self.debug_connect_parameters.reset_mode = reset_mode;
    }

    pub fn set_connection_mode(&mut self, connection_mode: DebugConnectMode) {
        self.debug_connect_parameters.connection_mode = connection_mode
    }
}

impl std::fmt::Display for STLink {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "\
Board: {},
Serial Number: {},
Firmware Version: {},
Index: {},
Connection Mode: {:?},
Reset Mode: {:?},
Access Port Count: {},
Access Port: {},
Debug Port: {:?},
Old Firmware: {},
{},
Frequency: {},
Bridge: {},
Shared: {},
Debug Sleep: {},
Speed: {},
Target Voltage: {}",
            self.board().unwrap_or("undefined".into()),
            self.serial_number().unwrap_or("undefined".into()),
            self.firmware_version().unwrap_or("undefined".into()),
            self.index(),
            self.connection_mode(),
            self.reset_mode(),
            self.access_port_count(),
            self.access_port(),
            self.debug_port(),
            self.old_firmware(),
            self.frequencies(),
            self.frequency(),
            self.bridge(),
            self.shared(),
            self.debug_sleep(),
            self.speed(),
            self.target_voltage().unwrap_or(0.0)
        )
    }
}

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    device_general_info: DeviceGeneralInfo,
}

impl DeviceInfo {
    pub fn category(&self) -> Result<String, err::Error> {
        Ok(String::from_utf8(
            self.device_general_info
                .category
                .iter()
                .map(|&c| c as u8)
                .collect(),
        )?.trim_matches(char::from(0)).to_owned())
    }

    pub fn cpu(&self) -> Result<String, err::Error> {
        Ok(String::from_utf8(
            self.device_general_info
                .cpu
                .iter()
                .map(|&c| c as u8)
                .collect(),
        )?.trim_matches(char::from(0)).to_owned())
    }

    pub fn name(&self) -> Result<String, err::Error> {
        Ok(String::from_utf8(
            self.device_general_info
                .name
                .iter()
                .map(|&c| c as u8)
                .collect(),
        )?.trim_matches(char::from(0)).to_owned())
    }

    pub fn series(&self) -> Result<String, err::Error> {
        Ok(String::from_utf8(
            self.device_general_info
                .series
                .iter()
                .map(|&c| c as u8)
                .collect(),
        )?.trim_matches(char::from(0)).to_owned())
    }

    pub fn description(&self) -> Result<String, err::Error> {
        Ok(String::from_utf8(
            self.device_general_info
                .description
                .iter()
                .map(|&c| c as u8)
                .collect(),
        )?.trim_matches(char::from(0)).to_owned())
    }

    pub fn revision_id(&self) -> Result<String, err::Error> {
        Ok(String::from_utf8(
            self.device_general_info
                .revision_id
                .iter()
                .map(|&c| c as u8)
                .collect(),
        )?.trim_matches(char::from(0)).to_owned())
    }

    pub fn board(&self) -> Result<String, err::Error> {
        Ok(String::from_utf8(
            self.device_general_info
                .board
                .iter()
                .map(|&c| c as u8)
                .collect(),
        )?.trim_matches(char::from(0)).to_owned())
    }

    pub fn device_id(&self) -> i32 {
        self.device_general_info.device_id.into()
    }

    pub fn flash_size(&self) -> i32 {
        self.device_general_info.flash_size
    }

    pub fn bootloader_version(&self) -> i32 {
        self.device_general_info.bootloader_version
    }
}

impl std::fmt::Display for DeviceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "\
Category: {},
Cpu: {},
Name: {},
Series: {},
Description: {},
Revision Id: {},
Board: {},
Device Id: 0x{:X},
Flash Size: 0x{:X},
Bootloader Version: 0x{:X}",
            self.category().unwrap_or("undefined".into()),
            self.cpu().unwrap_or("undefined".into()),
            self.name().unwrap_or("undefined".into()),
            self.series().unwrap_or("undefined".into()),
            self.description().unwrap_or("undefined".into()),
            self.revision_id().unwrap_or("undefined".into()),
            self.board().unwrap_or("undefined".into()),
            self.device_id(),
            self.flash_size(),
            self.bootloader_version()
        )
    }
}

pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    SP,
    LR,
    PC,
}

impl From<Register> for u32 {
    fn from(register: Register) -> Self {
        match register {
            Register::R0 => 0,
            Register::R1 => 1,
            Register::R2 => 2,
            Register::R3 => 3,
            Register::R4 => 4,
            Register::R5 => 5,
            Register::R6 => 6,
            Register::R7 => 7,
            Register::R8 => 8,
            Register::R9 => 9,
            Register::R10 => 10,
            Register::R11 => 11,
            Register::R12 => 12,
            Register::SP => 13,
            Register::LR => 14,
            Register::PC => 15,
        }
    }
}

pub struct STM32CubeProg {
    #[allow(dead_code)]
    library: libloading::Library,
    vtable: VTable,
}

impl STM32CubeProg {
    #[cfg(unix)]
    fn library_path(path: &std::path::Path) -> std::path::PathBuf {
        path.join("lib/libCubeProgrammer_API.so")
    }

    #[cfg(windows)]
    fn library_path(path: &std::path::Path) -> std::path::PathBuf {
        path.join("api\\lib\\CubeProgrammer_API.dll")
    }

    #[cfg(unix)]
    fn flashloader_path(path: &std::path::Path) -> std::path::PathBuf {
        path.join("bin")
    }

    #[cfg(windows)]
    fn flashloader_path(path: &std::path::Path) -> std::path::PathBuf {
        path.join("bin")
    }

    #[cfg(unix)]
    fn load_library(path: &std::path::Path) -> Result<libloading::Library, err::Error> {
        let library: libloading::Library =
            unsafe { libloading::os::unix::Library::new(path)?.into() };
        Ok(library)
    }

    #[cfg(windows)]
    fn load_library(path: &std::path::Path) -> Result<libloading::Library, err::Error> {
        let library: libloading::Library = unsafe {
            libloading::os::windows::Library::load_with_flags(
                path,
                libloading::os::windows::LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR
                    | libloading::os::windows::LOAD_LIBRARY_SEARCH_SYSTEM32
                    | libloading::os::windows::LOAD_LIBRARY_SEARCH_DEFAULT_DIRS,
            )?
            .into()
        };
        Ok(library)
    }

    pub fn new<P: AsRef<std::path::Path>>(path: P) -> Result<Self, err::Error> {
        let library_path = Self::library_path(path.as_ref());
        let library = Self::load_library(library_path.as_ref())?;
        let vtable = VTable::new(&library)?;

        unsafe {
            (vtable.set_loaders_path)(
                Self::flashloader_path(path.as_ref())
                    .as_os_str()
                    .as_encoded_bytes()
                    .as_ptr() as *const i8,
            )
        };

        let cb: DisplayCallbacks = DisplayCallbacks {
            init_progress_bar: init_progress_bar,
            log_message: log_message,
            load_bar: load_bar,
        };

        unsafe { (vtable.set_display_callbacks)(cb) };

        unsafe { (vtable.set_verbosity_level)(Verbosity::Level0) };

        Ok(STM32CubeProg { library, vtable })
    }

    pub fn discover(&self) -> Result<Vec<STLink>, err::Error> {
        let mut debug_connect_parameters = std::ptr::null_mut();
        let stlink_count =
            unsafe { (self.vtable.get_stlink_list)(&mut debug_connect_parameters, 0) };

        let params_slice =
            unsafe { std::slice::from_raw_parts(debug_connect_parameters, stlink_count as usize) };

        params_slice
            .iter()
            .map(|param| -> Result<STLink, err::Error> {
                let debug_connect_parameters = param.clone();
                Ok(STLink {
                    debug_connect_parameters,
                })
            })
            .collect::<Result<Vec<STLink>, err::Error>>()
    }

    pub fn connect(&self, stlink: &STLink) -> Result<(), err::Error> {
        let error = unsafe { (self.vtable.connect_stlink)(stlink.debug_connect_parameters) };
        if error == 0 {
            Ok(())
        } else {
            Err(err::CubeProgrammerError::from(error).into())
        }
    }

    pub fn disconnect(&self) {
        unsafe { (self.vtable.disconnect)() };
    }

    pub fn reset(&self, stlink: &STLink) -> Result<(), err::Error> {
        let error = unsafe { (self.vtable.reset)(stlink.debug_connect_parameters.reset_mode) };
        if error == 0 {
            Ok(())
        } else {
            Err(err::CubeProgrammerError::from(error).into())
        }
    }

    pub fn mass_erase(&self) -> Result<(), err::Error> {
        let error = unsafe { (self.vtable.mass_erase)() };
        if error == 0 {
            Ok(())
        } else {
            Err(err::CubeProgrammerError::from(error).into())
        }
    }

    pub fn download<P: AsRef<std::path::Path>>(
        &self,
        path: P,
        address: Option<u32>,
        skip_erase: Option<bool>,
        verify: Option<bool>,
    ) -> Result<(), err::Error> {
        let c_path = widestring::WideCString::from_os_str(
            std::fs::canonicalize(path.as_ref())?.as_os_str(),
        )?;

        let error = unsafe {
            (self.vtable.download_file)(
                c_path.as_ptr(),
                address.unwrap_or(0),
                skip_erase.unwrap_or(true).into(),
                verify.unwrap_or(true).into(),
                std::ptr::null(),
            )
        };
        if error == 0 {
            Ok(())
        } else {
            Err(err::CubeProgrammerError::from(error).into())
        }
    }

    pub fn device_info(&self) -> Result<DeviceInfo, err::Error> {
        let device_general_info = unsafe { (self.vtable.get_device_general_info)().as_ref() };

        match device_general_info {
            Some(value) => Ok(DeviceInfo {
                device_general_info: value.clone(),
            }),
            None => Err(err::CubeProgrammerError::NoDeviceFound.into()),
        }
    }

    pub fn read_core_register(&self, register: Register) -> Result<u32, err::Error> {
        let mut data = 0;
        let error = unsafe { (self.vtable.read_core_register)(register.into(), &mut data) };
        if error == 0 {
            Ok(data)
        } else {
            Err(err::CubeProgrammerError::from(error).into())
        }
    }

    pub fn write_core_register(&self, register: Register, data: u32) -> Result<(), err::Error> {
        let error = unsafe { (self.vtable.write_core_register)(register.into(), data) };
        if error == 0 {
            Ok(())
        } else {
            Err(err::CubeProgrammerError::from(error).into())
        }
    }

    pub fn read_memory8(&self, address: u32, size: u32) -> Result<Vec<u8>, err::Error> {
        let mut data = std::ptr::null_mut();
        let error = unsafe { (self.vtable.read_memory)(address, &mut data, size) };
        if error == 0 {
            let data: &mut [u8] = unsafe { core::slice::from_raw_parts_mut(data, size as usize) };
            Ok(data.to_vec())
        } else {
            Err(err::CubeProgrammerError::from(error).into())
        }
    }

    pub fn read_memory32(&self, address: u32, size: u32) -> Result<Vec<u32>, err::Error> {
        let mut data = std::ptr::null_mut();
        let error = unsafe { (self.vtable.read_memory)(address, &mut data, size * 4) };
        if error == 0 {
            let data: &mut [u8] =
                unsafe { core::slice::from_raw_parts_mut(data, (size * 4) as usize) };
            data.chunks(4)
                .map(|chunk| -> Result<u32, err::Error> {
                    let chunk = std::convert::TryInto::try_into(chunk)?;
                    Ok(u32::from_le_bytes(chunk))
                })
                .collect::<Result<Vec<u32>, err::Error>>()
        } else {
            Err(err::CubeProgrammerError::from(error).into())
        }
    }

    pub fn write_memory8(&self, address: u32, data: Vec<u8>) -> Result<(), err::Error> {
        let size: u32 = std::convert::TryInto::try_into(data.len())?;

        let error = unsafe { (self.vtable.write_memory)(address, data.clone().as_mut_ptr(), size) };
        if error == 0 {
            Ok(())
        } else {
            Err(err::CubeProgrammerError::from(error).into())
        }
    }

    pub fn write_memory32(&self, address: u32, data_u32: Vec<u32>) -> Result<(), err::Error> {
        let size: u32 = std::convert::TryInto::try_into(data_u32.len())?;
        let mut data_u8: Vec<u8> = Vec::new();
        for &num in &data_u32 {
            data_u8.extend_from_slice(&num.to_le_bytes());
        }

        let error = unsafe { (self.vtable.write_memory)(address, data_u8.as_mut_ptr(), size * 4) };
        if error == 0 {
            Ok(())
        } else {
            Err(err::CubeProgrammerError::from(error).into())
        }
    }
}
