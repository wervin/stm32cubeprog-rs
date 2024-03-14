use std::convert::From;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum Error {
    LibLoadingError(libloading::Error),
    FormatError(std::fmt::Error),
    FromUtf8Error(std::string::FromUtf8Error),
    CubeProgrammerError(CubeProgrammerError),
    IoError(std::io::Error),
    WideStringError(widestring::error::ContainsNul<crate::wchar>),
    UnsupportedPlatform,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            self::Error::LibLoadingError(e) => write!(f, "Lib loading error: {}", e),
            self::Error::FormatError(e) => write!(f, "Format error: {}", e),
            self::Error::FromUtf8Error(e) => write!(f, "UTF-8 conversion error: {}", e),
            self::Error::UnsupportedPlatform => {
                write!(f, ": The target system is not supported by visa")
            }
            self::Error::CubeProgrammerError(e) => write!(f, "Cube Programmer error: {}", e),
            self::Error::IoError(e) => write!(f, "IO error: {}", e),
            self::Error::WideStringError(e) => write!(f, "Wide string error: {}", e),
        }
    }
}

impl From<libloading::Error> for Error {
    fn from(err: libloading::Error) -> Error {
        Error::LibLoadingError(err)
    }
}

impl From<std::fmt::Error> for Error {
    fn from(err: std::fmt::Error) -> Error {
        Error::FormatError(err)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Error {
        Error::FromUtf8Error(err)
    }
}

impl From<CubeProgrammerError> for Error {
    fn from(err: CubeProgrammerError) -> Error {
        Error::CubeProgrammerError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<widestring::error::ContainsNul<crate::wchar>> for Error {
    fn from(err: widestring::error::ContainsNul<crate::wchar>) -> Error {
        Error::WideStringError(err)
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Copy, Clone)]
pub enum CubeProgrammerError {
    DeviceNotConnected = -1,
    NoDeviceFound = -2,
    ConnectionError = -3,
    FileNotFound = -4,
    UnsupportedOperation = -5,
    UnsupportedInterface = -6,
    InsufficientMemory = -7,
    UnknownParameters = -8,
    MemoryReadError = -9,
    MemoryWriteError = -10,
    MemoryEraseError = -11,
    UnsupportedFileFormat = -12,
    RefreshRequired = -13,
    SecurityError = -14,
    FrequencyError = -15,
    RdpEnabledError = -16,
    UnknownError = -17,
}

impl Display for CubeProgrammerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            CubeProgrammerError::DeviceNotConnected => write!(f, "Device not connected"),
            CubeProgrammerError::NoDeviceFound => write!(f, "Device not found"),
            CubeProgrammerError::ConnectionError => write!(f, "Device connection error"),
            CubeProgrammerError::FileNotFound => write!(f, "No such file"),
            CubeProgrammerError::UnsupportedOperation => write!(
                f,
                "Operation not supported or unimplemented on this interface"
            ),
            CubeProgrammerError::UnsupportedInterface => write!(
                f,
                "Interface not supported or unimplemented on this plateform"
            ),
            CubeProgrammerError::InsufficientMemory => write!(f, "Insufficient memory"),
            CubeProgrammerError::UnknownParameters => write!(f, "Wrong parameters"),
            CubeProgrammerError::MemoryReadError => write!(f, "Memory read failure"),
            CubeProgrammerError::MemoryWriteError => write!(f, "Memory write failure"),
            CubeProgrammerError::MemoryEraseError => write!(f, "Memory erase failure"),
            CubeProgrammerError::UnsupportedFileFormat => {
                write!(f, "File format not supported for this kind of device")
            }
            CubeProgrammerError::RefreshRequired => write!(f, "Refresh required"),
            CubeProgrammerError::SecurityError => write!(f, "Security error"),
            CubeProgrammerError::FrequencyError => write!(f, "Frequency error"),
            CubeProgrammerError::RdpEnabledError => write!(f, "RDP Enabled error"),
            CubeProgrammerError::UnknownError => write!(f, "Unknown error"),
        }
    }
}

impl From<i32> for CubeProgrammerError {
    fn from(value: i32) -> CubeProgrammerError {
        match value {
            -1 => CubeProgrammerError::DeviceNotConnected,
            -2 => CubeProgrammerError::NoDeviceFound,
            -3 => CubeProgrammerError::ConnectionError,
            -4 => CubeProgrammerError::FileNotFound,
            -5 => CubeProgrammerError::UnsupportedOperation,
            -6 => CubeProgrammerError::UnsupportedInterface,
            -7 => CubeProgrammerError::InsufficientMemory,
            -8 => CubeProgrammerError::UnknownParameters,
            -9 => CubeProgrammerError::MemoryReadError,
            -10 => CubeProgrammerError::MemoryWriteError,
            -11 => CubeProgrammerError::MemoryEraseError,
            -12 => CubeProgrammerError::UnsupportedFileFormat,
            -13 => CubeProgrammerError::RefreshRequired,
            -14 => CubeProgrammerError::SecurityError,
            -15 => CubeProgrammerError::FrequencyError,
            -16 => CubeProgrammerError::RdpEnabledError,
            _ => CubeProgrammerError::UnknownError,
        }
    }
}

impl std::error::Error for CubeProgrammerError {}
