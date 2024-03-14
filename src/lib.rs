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
        })
    }
}

#[derive(Debug, Clone)]
pub struct STLink {
    debug_connect_parameters: DebugConnectParameters,
}

impl STLink {
    pub fn serial_number(&self) -> Result<String, err::Error> {
        Ok(String::from_utf8(
            self.debug_connect_parameters
                .serial_number
                .iter()
                .map(|&c| c as u8)
                .collect(),
        )?)
    }

    pub fn firmware_version(&self) -> Result<String, err::Error> {
        Ok(String::from_utf8(
            self.debug_connect_parameters
                .firmware_version
                .iter()
                .map(|&c| c as u8)
                .collect(),
        )?)
    }

    pub fn board(&self) -> Result<String, err::Error> {
        Ok(String::from_utf8(
            self.debug_connect_parameters
                .board
                .iter()
                .map(|&c| c as u8)
                .collect(),
        )?)
    }

    pub fn reset_mode(&mut self, reset_mode: DebugResetMode) {
        self.debug_connect_parameters.reset_mode = reset_mode;
    }

    pub fn connection_mode(&mut self, connection_mode: DebugConnectMode) {
        self.debug_connect_parameters.connection_mode = connection_mode
    }
}

impl std::fmt::Display for STLink {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Serial Number: {}, Firmware Version: {}, Board: {}",
            self.serial_number().unwrap_or("undefined".into()),
            self.firmware_version().unwrap_or("undefined".into()),
            self.board().unwrap_or("undefined".into()),
        )
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
        path.join("lib/libCubeProgrammer_API.so")
    }

    #[cfg(unix)]
    fn flashloader_path(path: &std::path::Path) -> std::path::PathBuf {
        path.join("bin")
    }

    #[cfg(windows)]
    fn flashloader_path(path: &std::path::Path) -> std::path::PathBuf {
        path.join("bin")
    }

    pub fn new<P: AsRef<std::path::Path>>(path: P) -> Result<Self, err::Error> {
        let library = unsafe { libloading::Library::new(Self::library_path(path.as_ref()))? };
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
        let mut debug_connect_parameters = 0 as *mut DebugConnectParameters;
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn failed_to_load_libray() {
        let stm32prog = super::STM32CubeProg::new("Oups");
        assert!(matches!(stm32prog, Err(_)));
    }

    #[test]
    #[cfg(unix)]
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
