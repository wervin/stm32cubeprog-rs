pub mod err;

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
    pub jtag_freq_number: std::os::raw::c_uint,
    pub swd_freq: [std::os::raw::c_uint; 12usize],
    pub swd_freq_number: std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DebugConnectParameters {
    pub dbg_port: DebugPort,
    pub index: std::os::raw::c_int,
    pub serial_number: [std::os::raw::c_char; 33usize],
    pub firmware_version: [std::os::raw::c_char; 20usize],
    pub target_voltage: [std::os::raw::c_char; 5usize],
    pub access_port_number: std::os::raw::c_int,
    pub access_port: std::os::raw::c_int,
    pub connection_mode: DebugConnectMode,
    pub reset_mode: DebugResetMode,
    pub is_old_firmware: std::os::raw::c_int,
    pub freq: Frequencies,
    pub frequency: std::os::raw::c_int,
    pub is_bridge: std::os::raw::c_int,
    pub shared: std::os::raw::c_int,
    pub board: [std::os::raw::c_char; 100usize],
    pub dbg_sleep: std::os::raw::c_int,
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
    unsafe extern "C" fn(debug_connect_parameters: DebugConnectParameters) -> ::std::os::raw::c_int;

pub struct VTable {
    set_loaders_path: libloading::os::unix::Symbol<SetLoaderPath>,
    set_display_callbacks: libloading::os::unix::Symbol<SetDisplayCallbacks>,
    set_verbosity_level: libloading::os::unix::Symbol<SetVerbosityLevel>,
    get_stlink_list: libloading::os::unix::Symbol<GetStLinkList>,
    connect_stlink: libloading::os::unix::Symbol<ConnectStLink>,
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

        Ok(VTable {
            set_loaders_path,
            set_display_callbacks,
            set_verbosity_level,
            get_stlink_list,
            connect_stlink,
        })
    }
}

pub struct STLink {
    serial_number: String,
    firmware_version: String,
    board: String,
    debug_connect_parameters: DebugConnectParameters,
}

impl std::fmt::Display for STLink {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Serial Number: {}, Firmware Version: {}, Board: {}",
            self.serial_number, self.firmware_version, self.board
        )
    }
}

pub struct STM32CubeProg {
    #[allow(dead_code)]
    library: libloading::Library,
    vtable: VTable,
}

impl STM32CubeProg {
    #[cfg(target_os = "linux")]
    fn library_path(path: &str) -> Result<String, std::fmt::Error> {
        Ok(format!("{path}/lib/libCubeProgrammer_API.so"))
    }

    #[cfg(target_os = "windows")]
    fn library_path(path: &str) -> Result<String, std::fmt::Error> {
        Ok(format!("{path}/lib/libCubeProgrammer_API.so"))
    }

    #[cfg(all(not(target_os = "linux"), not(target_os = "windows")))]
    fn library_path(_path: &str) -> Result<String, std::fmt::Error> {
        Err(Box::new(std::io::Error::from(
            std::io::ErrorKind::Unsupported,
        )))
    }

    #[cfg(target_os = "linux")]
    fn flashloader_path(path: &str) -> Result<String, std::fmt::Error> {
        Ok(format!("{path}/bin"))
    }

    #[cfg(target_os = "windows")]
    fn flashloader_path(path: &str) -> Result<String, std::fmt::Error> {
        Ok(format!("{path}/bin"))
    }

    #[cfg(all(not(target_os = "linux"), not(target_os = "windows")))]
    fn flashloader_path(_path: &str) -> Result<String, std::fmt::Error> {
        Err(Box::new(std::io::Error::from(
            std::io::ErrorKind::Unsupported,
        )))
    }

    pub fn new(path: &str) -> Result<Self, err::Error> {
        let library = unsafe { libloading::Library::new(Self::library_path(path)?)? };
        let vtable = VTable::new(&library)?;

        unsafe {
            (vtable.set_loaders_path)(Self::flashloader_path(path)?.as_bytes().as_ptr() as *const i8)
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

    pub fn discover(self) -> Result<Vec<STLink>, err::Error> {
        let mut debug_connect_parameters = 0 as *mut DebugConnectParameters;
        let stlink_count =
            unsafe { (self.vtable.get_stlink_list)(&mut debug_connect_parameters, 0) };

        let params_slice =
            unsafe { std::slice::from_raw_parts(debug_connect_parameters, stlink_count as usize) };

        params_slice
            .iter()
            .map(|param| -> Result<STLink, err::Error> {
                let serial_number =
                    String::from_utf8(param.serial_number.iter().map(|&c| c as u8).collect())?;
                let firmware_version =
                    String::from_utf8(param.firmware_version.iter().map(|&c| c as u8).collect())?;
                let board = String::from_utf8(param.board.iter().map(|&c| c as u8).collect())?;
                let debug_connect_parameters = param.clone();
                Ok(STLink {
                    serial_number,
                    firmware_version,
                    board,
                    debug_connect_parameters,
                })
            })
            .collect::<Result<Vec<STLink>, err::Error>>()
    }

    pub fn connect(self, stlink: STLink) -> Result<(), err::Error> {
        let error = unsafe { (self.vtable.connect_stlink)(stlink.debug_connect_parameters) };
        if error == 0 {
            Ok(())
        } else {
            Err(err::CubeProgrammerError::from(error).into())
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn failed_to_load_libray() {
        let stm32prog = super::STM32CubeProg::new("Oups");
        assert!(matches!(stm32prog, Err(_)));
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn open_library() {
        let home_dir = std::env::var_os("HOME")
            .map(std::path::PathBuf::from)
            .expect("Failed to get home directory, $HOME variable missing");
        let binding =
            home_dir.join("Applications/STMicroelectronics/STM32Cube/STM32CubeProgrammer");
        let stm32prog_path = binding
            .to_str()
            .expect("Failed to join STM32CubeProgrammer path");
        let stm32prog = super::STM32CubeProg::new(stm32prog_path);
        assert!(matches!(stm32prog, Ok(_)));
    }
}
