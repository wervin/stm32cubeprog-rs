#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
// #![allow(unused)]

pub type wchar_t = ::std::os::raw::c_int;

#[doc = " \\struct  bankSector.\n \\brief   This stucture indicates the sectors parameters."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bankSector {
    #[doc = "< Index of the sector."]
    pub index: ::std::os::raw::c_uint,
    #[doc = "< Sector size."]
    pub size: ::std::os::raw::c_uint,
    #[doc = "< Sector starting address."]
    pub address: ::std::os::raw::c_uint,
}

#[doc = " \\struct  deviceBank.\n \\brief   This stucture defines the memory sectors for each bank."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct deviceBank {
    #[doc = "< Number of sectors of the considered bank."]
    pub sectorsNumber: ::std::os::raw::c_uint,
    #[doc = "< Sectors specifications #Bank_Sector."]
    pub sectors: *mut bankSector,
}

#[doc = " \\struct  storageStructure.\n \\brief   This stucture describes sotrage characterization."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct storageStructure {
    #[doc = "< Number of exsisted banks."]
    pub banksNumber: ::std::os::raw::c_uint,
    #[doc = "< Banks sectors definition #Device_Bank."]
    pub banks: *mut deviceBank,
}

#[doc = " \\struct  bitCoefficient_C.\n \\brief   This stucture indicates the coefficients to access to the adequate option bit."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bitCoefficient_C {
    #[doc = "< Bit multiplier."]
    pub multiplier: ::std::os::raw::c_uint,
    #[doc = "< Bit offset."]
    pub offset: ::std::os::raw::c_uint,
}

#[doc = " \\struct  bitValue_C.\n \\brief   This stucture describes the option Bit value."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bitValue_C {
    #[doc = "< Option bit value."]
    pub value: ::std::os::raw::c_uint,
    #[doc = "< Option bit description."]
    pub description: [::std::os::raw::c_char; 1000usize],
}

#[doc = " \\struct  bit_C.\n \\brief   This stucture will be filled by values which characterize the device's option bytes.\n \\note    See product reference manual for more details."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bit_C {
    #[doc = "< Bit name such as RDP, BOR_LEV, nBOOT0..."]
    pub name: [::std::os::raw::c_char; 32usize],
    #[doc = "< Config description."]
    pub description: [::std::os::raw::c_char; 1000usize],
    #[doc = "< Word offset."]
    pub wordOffset: ::std::os::raw::c_uint,
    #[doc = "< Bit offset."]
    pub bitOffset: ::std::os::raw::c_uint,
    #[doc = "< Number of bits build the option."]
    pub bitWidth: ::std::os::raw::c_uint,
    #[doc = "< Access Read/Write."]
    pub access: ::std::os::raw::c_uchar,
    #[doc = "< Number of possible values."]
    pub valuesNbr: ::std::os::raw::c_uint,
    #[doc = "< Bits value, #BitValue_C."]
    pub values: *mut *mut bitValue_C,
    #[doc = "< Bits equation, #BitCoefficient_C."]
    pub equation: bitCoefficient_C,
    pub reference: *mut ::std::os::raw::c_uchar,
    pub bitValue: ::std::os::raw::c_uint,
}

#[doc = " \\struct  category_C\n \\brief   Get option bytes banks categories descriptions."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct category_C {
    #[doc = "< Get category name such as Read Out Protection, BOR Level..."]
    pub name: [::std::os::raw::c_char; 100usize],
    #[doc = "< Get bits number of the considered category."]
    pub bitsNbr: ::std::os::raw::c_uint,
    #[doc = "< Get internal bits descriptions."]
    pub bits: *mut *mut bit_C,
}

#[doc = " \\struct  bank_C\n \\brief   Get option bytes banks internal descriptions.\n \\note    STLINK and Bootloader interfaces have different addresses to access to option bytes registres."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bank_C {
    #[doc = "< Bank size."]
    pub size: ::std::os::raw::c_uint,
    #[doc = "< Bank starting address."]
    pub address: ::std::os::raw::c_uint,
    #[doc = "< Bank access Read/Write."]
    pub access: ::std::os::raw::c_uchar,
    #[doc = "< Number of option bytes categories."]
    pub categoriesNbr: ::std::os::raw::c_uint,
    #[doc = "< Get bank categories descriptions #Category_C."]
    pub categories: *mut *mut category_C,
}

#[doc = " \\struct  peripheral_C\n \\brief   Get peripheral option bytes general informations."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct peripheral_C {
    #[doc = "< Peripheral name."]
    pub name: [::std::os::raw::c_char; 32usize],
    #[doc = "< Peripheral description."]
    pub description: [::std::os::raw::c_char; 1000usize],
    #[doc = "< Number of existed banks."]
    pub banksNbr: ::std::os::raw::c_uint,
    #[doc = "< Get banks descriptions #Bank_C."]
    pub banks: *mut *mut bank_C,
}

#[doc = " no messages ever printed by the library"]
pub const cubeProgrammerVerbosityLevel_CUBEPROGRAMMER_VER_LEVEL_NONE: cubeProgrammerVerbosityLevel =
    0;
#[doc = " warning, error and success messages are printed (default)"]
pub const cubeProgrammerVerbosityLevel_CUBEPROGRAMMER_VER_LEVEL_ONE: cubeProgrammerVerbosityLevel =
    1;
#[doc = " error roots informational messages are printed"]
pub const cubeProgrammerVerbosityLevel_CUBEPROGRAMMER_VER_LEVEL_TWO: cubeProgrammerVerbosityLevel =
    2;
#[doc = " debug and informational messages are printed"]
pub const cubeProgrammerVerbosityLevel_CUBEPROGRAMMER_VER_LEVEL_DEBUG:
    cubeProgrammerVerbosityLevel = 3;
#[doc = " no progress bar is printed in the output of the library"]
pub const cubeProgrammerVerbosityLevel_CUBEPROGRAMMER_NO_PROGRESS_BAR:
    cubeProgrammerVerbosityLevel = 4;
#[doc = " \\enum  cubeProgrammerVerbosityLevel\n \\brief List of verbosity levels."]
pub type cubeProgrammerVerbosityLevel = ::std::os::raw::c_uint;
#[doc = " Success (no error)"]
pub const cubeProgrammerError_CUBEPROGRAMMER_NO_ERROR: cubeProgrammerError = 0;
#[doc = " Device not connected"]
pub const cubeProgrammerError_CUBEPROGRAMMER_ERROR_NOT_CONNECTED: cubeProgrammerError = -1;
#[doc = " Device not found"]
pub const cubeProgrammerError_CUBEPROGRAMMER_ERROR_NO_DEVICE: cubeProgrammerError = -2;
#[doc = " Device connection error"]
pub const cubeProgrammerError_CUBEPROGRAMMER_ERROR_CONNECTION: cubeProgrammerError = -3;
#[doc = " No such file"]
pub const cubeProgrammerError_CUBEPROGRAMMER_ERROR_NO_FILE: cubeProgrammerError = -4;
#[doc = " Operation not supported or unimplemented on this interface"]
pub const cubeProgrammerError_CUBEPROGRAMMER_ERROR_NOT_SUPPORTED: cubeProgrammerError = -5;
#[doc = " Interface not supported or unimplemented on this plateform"]
pub const cubeProgrammerError_CUBEPROGRAMMER_ERROR_INTERFACE_NOT_SUPPORTED: cubeProgrammerError =
    -6;
#[doc = " Insufficient memory"]
pub const cubeProgrammerError_CUBEPROGRAMMER_ERROR_NO_MEM: cubeProgrammerError = -7;
#[doc = " Wrong parameters"]
pub const cubeProgrammerError_CUBEPROGRAMMER_ERROR_WRONG_PARAM: cubeProgrammerError = -8;
#[doc = " Memory read failure"]
pub const cubeProgrammerError_CUBEPROGRAMMER_ERROR_READ_MEM: cubeProgrammerError = -9;
#[doc = " Memory write failure"]
pub const cubeProgrammerError_CUBEPROGRAMMER_ERROR_WRITE_MEM: cubeProgrammerError = -10;
#[doc = " Memory erase failure"]
pub const cubeProgrammerError_CUBEPROGRAMMER_ERROR_ERASE_MEM: cubeProgrammerError = -11;
#[doc = " File format not supported for this kind of device"]
pub const cubeProgrammerError_CUBEPROGRAMMER_ERROR_UNSUPPORTED_FILE_FORMAT: cubeProgrammerError =
    -12;
#[doc = " Refresh required"]
pub const cubeProgrammerError_CUBEPROGRAMMER_ERROR_REFRESH_REQUIRED: cubeProgrammerError = -13;
#[doc = " Refresh required"]
pub const cubeProgrammerError_CUBEPROGRAMMER_ERROR_NO_SECURITY: cubeProgrammerError = -14;
#[doc = " Changing frequency problem"]
pub const cubeProgrammerError_CUBEPROGRAMMER_ERROR_CHANGE_FREQ: cubeProgrammerError = -15;
#[doc = " RDP Enabled error"]
pub const cubeProgrammerError_CUBEPROGRAMMER_ERROR_RDP_ENABLED: cubeProgrammerError = -16;
#[doc = " Other error"]
pub const cubeProgrammerError_CUBEPROGRAMMER_ERROR_OTHER: cubeProgrammerError = -99;
#[doc = " \\enum  cubeProgrammerError\n \\brief List of errors that can be occured."]
pub type cubeProgrammerError = ::std::os::raw::c_int;
pub const flashSize_Flash_Size_1KB: flashSize = 1024;
pub const flashSize_Flash_Size_512KB: flashSize = 524288;
pub const flashSize_Flash_Size_256KB: flashSize = 262144;
pub type flashSize = ::std::os::raw::c_uint;
pub const wbFunctionArguments_FIRST_INSTALL_ACTIVE: wbFunctionArguments = 1;
pub const wbFunctionArguments_FIRST_INSTALL_NOT_ACTIVE: wbFunctionArguments = 0;
pub const wbFunctionArguments_START_STACK_ACTIVE: wbFunctionArguments = 1;
pub const wbFunctionArguments_START_STACK_NOT_ACTIVE: wbFunctionArguments = 1;
pub const wbFunctionArguments_VERIFY_FILE_DOWLOAD_FILE: wbFunctionArguments = 1;
pub const wbFunctionArguments_DO_NOT_VERIFY_DOWLOAD_FILE: wbFunctionArguments = 0;
pub type wbFunctionArguments = ::std::os::raw::c_uint;
#[doc = "< Even parity bit."]
pub const usartParity_EVEN: usartParity = 0;
#[doc = "< Odd parity bit."]
pub const usartParity_ODD: usartParity = 1;
#[doc = "< No check parity."]
pub const usartParity_NONE: usartParity = 2;
#[doc = " \\enum  usartParity\n \\brief The parity bit in the data frame of the USART communication tells the receiving device if there is any error in the data bits."]
pub type usartParity = ::std::os::raw::c_uint;
#[doc = "< No flow control."]
pub const usartFlowControl_OFF: usartFlowControl = 0;
#[doc = "< Hardware flow control : RTS/CTS."]
pub const usartFlowControl_HARDWARE: usartFlowControl = 1;
#[doc = "< Software flow control : Transmission is started and stopped by sending special characters."]
pub const usartFlowControl_SOFTWARE: usartFlowControl = 2;
#[doc = " \\enum  usartFlowControl\n \\brief UART Flow Control is a method for devices to communicate with each other over UART without the risk of losing data."]
pub type usartFlowControl = ::std::os::raw::c_uint;

#[doc = " \\struct  dfuDeviceInfo\n \\brief   Get DFU device informations ."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dfuDeviceInfo {
    #[doc = "< USB index."]
    pub usbIndex: [::std::os::raw::c_char; 10usize],
    #[doc = "< Bus number."]
    pub busNumber: ::std::os::raw::c_int,
    #[doc = "< Address number."]
    pub addressNumber: ::std::os::raw::c_int,
    #[doc = "< Product number."]
    pub productId: [::std::os::raw::c_char; 100usize],
    #[doc = "< Serial number."]
    pub serialNumber: [::std::os::raw::c_char; 100usize],
    #[doc = "< DFU version."]
    pub dfuVersion: ::std::os::raw::c_uint,
}

#[doc = " \\struct  usartConnectParameters\n \\brief   Specify the USART connect parameters."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct usartConnectParameters {
    #[doc = "< Interface identifier: COM1, COM2, /dev/ttyS0..."]
    pub portName: [::std::os::raw::c_char; 100usize],
    #[doc = "< Speed transmission: 115200, 9600..."]
    pub baudrate: ::std::os::raw::c_uint,
    #[doc = "< Parity bit: value in usartParity."]
    pub parity: usartParity,
    #[doc = "< Data bit: value in {6, 7, 8}."]
    pub dataBits: ::std::os::raw::c_uchar,
    #[doc = "< Stop bit: value in {1, 1.5, 2}."]
    pub stopBits: f32,
    #[doc = "< Flow control: value in usartFlowControl."]
    pub flowControl: usartFlowControl,
    #[doc = "< RTS: Value in {0,1}."]
    pub statusRTS: ::std::os::raw::c_int,
    #[doc = "< DTR: Value in {0,1}."]
    pub statusDTR: ::std::os::raw::c_int,
    #[doc = "< Set No Init bits: value in {0,1}."]
    pub noinitBits: ::std::os::raw::c_uchar,
    #[doc = "< request a read unprotect: value in {0,1}."]
    pub rdu: ::std::os::raw::c_char,
    #[doc = "< request a TZEN regression: value in {0,1}."]
    pub tzenreg: ::std::os::raw::c_char,
}

#[doc = " \\struct  dfuConnectParameters\n \\brief   Specify the USB DFU connect parameters."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dfuConnectParameters {
    pub usb_index: *mut ::std::os::raw::c_char,
    #[doc = "< request a read unprotect: value in {0,1}."]
    pub rdu: ::std::os::raw::c_char,
    #[doc = "< request a TZEN regression: value in {0,1}."]
    pub tzenreg: ::std::os::raw::c_char,
}

#[doc = " \\struct  spiConnectParameters\n \\brief   Specify the SPI connect parameters.\n \\note    Recommended SPI parameters : baudrate=375, crcPol=7, direction=0, cpha=0, cpol=0, crc=0, firstBit=1, frameFormat=0, dataSize=1, mode=1, nss=1, nssPulse=1, delay=1"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct spiConnectParameters {
    #[doc = "< Speed transmission 187, 375, 750, 1500, 3000, 6000, 12000 KHz."]
    pub baudrate: u32,
    #[doc = "< crc polynom value."]
    pub crcPol: u16,
    #[doc = "< 2LFullDuplex/2LRxOnly/1LRx/1LTx."]
    pub direction: ::std::os::raw::c_int,
    #[doc = "< 1Edge or 2Edge."]
    pub cpha: ::std::os::raw::c_int,
    #[doc = "< LOW or HIGH."]
    pub cpol: ::std::os::raw::c_int,
    #[doc = "< DISABLE or ENABLE."]
    pub crc: ::std::os::raw::c_int,
    #[doc = "< First bit: LSB or MSB."]
    pub firstBit: ::std::os::raw::c_int,
    #[doc = "< Frame format: Motorola or TI."]
    pub frameFormat: ::std::os::raw::c_int,
    #[doc = "< Size of frame data: 16bit or 8bit ."]
    pub dataSize: ::std::os::raw::c_int,
    #[doc = "< Operating mode: Slave or Master."]
    pub mode: ::std::os::raw::c_int,
    #[doc = "< Selection: Soft or Hard."]
    pub nss: ::std::os::raw::c_int,
    #[doc = "< NSS pulse: No Pulse or Pulse."]
    pub nssPulse: ::std::os::raw::c_int,
    #[doc = "< Delay of few microseconds, No Delay or Delay, at least 4us delay is inserted"]
    pub delay: ::std::os::raw::c_int,
}

#[doc = " \\struct  canConnectParameters\n \\brief   Specify the CAN connect parameters.\n \\note    Not all configurations are supported by STM32 Bootloader, such as CAN type is STANDARD and the filter should be always activated.\n \\note    Recommended CAN parameters : br=125000, mode=0, ide=0, rtr=0, fifo=0, fm=0, fs=1, fe=1, fbn=0"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct canConnectParameters {
    #[doc = "< Baudrate and speed transmission 125KHz, 250KHz, 500KHz..."]
    pub br: ::std::os::raw::c_int,
    #[doc = "< CAN mode: NORMAL, LOOPBACK...,"]
    pub mode: ::std::os::raw::c_int,
    #[doc = "< CAN type: STANDARD or EXTENDED."]
    pub ide: ::std::os::raw::c_int,
    #[doc = "< Frame format: DATA or REMOTE."]
    pub rtr: ::std::os::raw::c_int,
    #[doc = "< Memory of received messages: FIFO0 or FIFO1."]
    pub fifo: ::std::os::raw::c_int,
    #[doc = "< Filter mode: MASK or LIST."]
    pub fm: ::std::os::raw::c_int,
    #[doc = "< Filter scale: 16 or 32."]
    pub fs: ::std::os::raw::c_int,
    #[doc = "< Filter activation: DISABLE or ENABLE."]
    pub fe: ::std::os::raw::c_int,
    #[doc = "< Filter bank number: 0 to 13."]
    pub fbn: ::std::os::raw::c_char,
}

#[doc = " \\struct  i2cConnectParameters\n \\brief   Specify the I2C connect parameters.\n \\warning The Bootloader Slave address varies depending on the device (see AN2606).\n \\note    Not all configurations are supported by STM32 Bootloader, such as address in 7 bits form, analog filter: ENABLE, digital filter: DISABLE.\n \\note    Recommended I2C parameters : add=0x??, br=400, sm=1, am=0, af=1, df=0, dnf=0, rt=0, ft=0"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct i2cConnectParameters {
    #[doc = "< Device address in hex format."]
    pub add: ::std::os::raw::c_int,
    #[doc = "< Baudrate and speed transmission : 100 or 400 KHz."]
    pub br: ::std::os::raw::c_int,
    #[doc = "< Speed Mode: STANDARD or FAST."]
    pub sm: ::std::os::raw::c_int,
    #[doc = "< Address Mode: 7 or 10 bits."]
    pub am: ::std::os::raw::c_int,
    #[doc = "< Analog filter: DISABLE or ENABLE."]
    pub af: ::std::os::raw::c_int,
    #[doc = "< Digital filter: DISABLE or ENABLE."]
    pub df: ::std::os::raw::c_int,
    #[doc = "< Digital noise filter: 0 to 15."]
    pub dnf: ::std::os::raw::c_char,
    #[doc = "< Rise time: 0-1000 for STANDARD speed mode and  0-300 for FAST."]
    pub rt: ::std::os::raw::c_int,
    #[doc = "< Fall time: 0-300 for STANDARD speed mode and  0-300 for FAST."]
    pub ft: ::std::os::raw::c_int,
}

#[doc = "< Apply a reset by the software."]
pub const debugResetMode_SOFTWARE_RESET: debugResetMode = 0;
#[doc = "< Apply a reset by the hardware."]
pub const debugResetMode_HARDWARE_RESET: debugResetMode = 1;
#[doc = "< Apply a reset by the internal core peripheral."]
pub const debugResetMode_CORE_RESET: debugResetMode = 2;
#[doc = " \\enum  debugResetMode\n \\brief Choose the way to apply a system reset."]
pub type debugResetMode = ::std::os::raw::c_uint;
#[doc = "< Connect with normal mode, the target is reset then halted while the type of reset is selected using the [debugResetMode]."]
pub const debugConnectMode_NORMAL_MODE: debugConnectMode = 0;
#[doc = "< Connect with hotplug mode,  this option allows the user to connect to the target without halt or reset."]
pub const debugConnectMode_HOTPLUG_MODE: debugConnectMode = 1;
#[doc = "< Connect with under reset mode, option allows the user to connect to the target using a reset vector catch before executing any instruction."]
pub const debugConnectMode_UNDER_RESET_MODE: debugConnectMode = 2;
#[doc = "< Connect with power down mode."]
pub const debugConnectMode_POWER_DOWN_MODE: debugConnectMode = 3;
#[doc = "< Connect with pre reset mode."]
pub const debugConnectMode_PRE_RESET_MODE: debugConnectMode = 4;
#[doc = " \\enum  debugConnectMode\n \\brief Choose the appropriate mode for connection."]
pub type debugConnectMode = ::std::os::raw::c_uint;
#[doc = "< JTAG debug port."]
pub const debugPort_JTAG: debugPort = 0;
#[doc = "< SWD debug port."]
pub const debugPort_SWD: debugPort = 1;
#[doc = " \\enum  debugPort\n \\brief Select the debug port interface for connection."]
pub type debugPort = ::std::os::raw::c_uint;
#[doc = " \\struct  frequencies\n \\brief   Get supported frequencies for JTAG and SWD ineterfaces."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct frequencies {
    #[doc = "<  JTAG frequency."]
    pub jtagFreq: [::std::os::raw::c_uint; 12usize],
    #[doc = "<  Get JTAG supported frequencies."]
    pub jtagFreqNumber: ::std::os::raw::c_uint,
    #[doc = "<  SWD frequency."]
    pub swdFreq: [::std::os::raw::c_uint; 12usize],
    #[doc = "<  Get SWD supported frequencies."]
    pub swdFreqNumber: ::std::os::raw::c_uint,
}

#[doc = " \\struct  debugConnectParameters\n \\brief   Get device characterization and specify connection parameters through ST-LINK interface."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct debugConnectParameters {
    #[doc = "< Select the type of debug interface #debugPort."]
    pub dbgPort: debugPort,
    #[doc = "< Select one of the debug ports connected."]
    pub index: ::std::os::raw::c_int,
    #[doc = "< ST-LINK serial number."]
    pub serialNumber: [::std::os::raw::c_char; 33usize],
    #[doc = "< Firmware version."]
    pub firmwareVersion: [::std::os::raw::c_char; 20usize],
    #[doc = "< Operate voltage."]
    pub targetVoltage: [::std::os::raw::c_char; 5usize],
    #[doc = "< Number of available access port."]
    pub accessPortNumber: ::std::os::raw::c_int,
    #[doc = "< Select access port controller."]
    pub accessPort: ::std::os::raw::c_int,
    #[doc = "< Select the debug CONNECT mode #debugConnectMode."]
    pub connectionMode: debugConnectMode,
    #[doc = "< Select the debug RESET mode #debugResetMode."]
    pub resetMode: debugResetMode,
    #[doc = "< Check Old ST-LINK firmware version."]
    pub isOldFirmware: ::std::os::raw::c_int,
    #[doc = "< Supported frequencies #frequencies."]
    pub freq: frequencies,
    #[doc = "< Select specific frequency."]
    pub frequency: ::std::os::raw::c_int,
    #[doc = "< Indicates if it's Bridge device or not."]
    pub isBridge: ::std::os::raw::c_int,
    #[doc = "< Select connection type, if it's shared, use ST-LINK Server."]
    pub shared: ::std::os::raw::c_int,
    #[doc = "< board Name"]
    pub board: [::std::os::raw::c_char; 100usize],
    pub DBG_Sleep: ::std::os::raw::c_int,
    #[doc = "< Select speed flashing of Cortex M33 series."]
    pub speed: ::std::os::raw::c_int,
}

#[doc = "< STLINK used as connection interface."]
pub const targetInterfaceType_STLINK_INTERFACE: targetInterfaceType = 0;
#[doc = "< USART used as connection interface."]
pub const targetInterfaceType_USART_INTERFACE: targetInterfaceType = 1;
#[doc = "< USB DFU used as connection interface."]
pub const targetInterfaceType_USB_INTERFACE: targetInterfaceType = 2;
#[doc = "< SPI used as connection interface."]
pub const targetInterfaceType_SPI_INTERFACE: targetInterfaceType = 3;
#[doc = "< I2C used as connection interface."]
pub const targetInterfaceType_I2C_INTERFACE: targetInterfaceType = 4;
#[doc = "< CAN used as connection interface."]
pub const targetInterfaceType_CAN_INTERFACE: targetInterfaceType = 5;
#[doc = " \\enum  targetInterfaceType\n \\brief Indicates the supported interfaces."]
pub type targetInterfaceType = ::std::os::raw::c_uint;
#[doc = " \\struct  displayCallBacks\n \\brief   Functions must be implemented to personalize the display of messages."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct displayCallBacks {
    #[doc = "< Add a progress bar."]
    pub initProgressBar: ::std::option::Option<unsafe extern "C" fn()>,
    #[doc = "< Display internal messages according to verbosity level."]
    pub logMessage: ::std::option::Option<
        unsafe extern "C" fn(msgType: ::std::os::raw::c_int, str_: *const wchar_t),
    >,
    #[doc = "< Display the loading of read/write process."]
    pub loadBar: ::std::option::Option<
        unsafe extern "C" fn(x: ::std::os::raw::c_int, n: ::std::os::raw::c_int),
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct segmentData_C {
    #[doc = "< Segment start address."]
    pub address: ::std::os::raw::c_int,
    #[doc = "< Memory segment size."]
    pub size: ::std::os::raw::c_int,
    #[doc = "< Memory segment data."]
    pub data: *mut ::std::os::raw::c_uchar,
}

#[doc = " \\struct  FileData_C\n \\brief   Get file required informations."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fileData_C {
    #[doc = "< File extension type."]
    pub Type: ::std::os::raw::c_int,
    #[doc = "< Number of required segments."]
    pub segmentsNbr: ::std::os::raw::c_int,
    #[doc = "< Segments description."]
    pub segments: *mut segmentData_C,
}

#[doc = " \\struct  GeneralInf\n \\brief   Get device general informations."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct generalInf {
    #[doc = "< Device ID."]
    pub deviceId: ::std::os::raw::c_ushort,
    #[doc = "< Flash memory size."]
    pub flashSize: ::std::os::raw::c_int,
    #[doc = "< Bootloader version"]
    pub bootloaderVersion: ::std::os::raw::c_int,
    #[doc = "< Device MCU or MPU."]
    pub type_: [::std::os::raw::c_char; 4usize],
    #[doc = "< Cortex CPU."]
    pub cpu: [::std::os::raw::c_char; 20usize],
    #[doc = "< Device name."]
    pub name: [::std::os::raw::c_char; 100usize],
    #[doc = "< Device serie."]
    pub series: [::std::os::raw::c_char; 100usize],
    #[doc = "< Take notice."]
    pub description: [::std::os::raw::c_char; 150usize],
    #[doc = "< Revision ID."]
    pub revisionId: [::std::os::raw::c_char; 8usize],
    #[doc = "< Board Rpn."]
    pub board: [::std::os::raw::c_char; 100usize],
}

#[doc = " \\struct  deviceSector\n \\brief   Get device sectors basic informations."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct deviceSector {
    #[doc = "< Number of Sectors."]
    pub sectorNum: u32,
    #[doc = "< Sector Size in BYTEs."]
    pub sectorSize: u32,
}

#[doc = " \\struct  externalLoader\n \\brief   Get external Loader parameters to launch the process of programming an external flash memory."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct externalLoader {
    #[doc = "< FlashLoader file path."]
    pub filePath: [::std::os::raw::c_char; 200usize],
    #[doc = "< Device Name and Description."]
    pub deviceName: [::std::os::raw::c_char; 100usize],
    #[doc = "< Device Type: ONCHIP, EXT8BIT, EXT16BIT, ..."]
    pub deviceType: ::std::os::raw::c_int,
    #[doc = "< Default Device Start Address."]
    pub deviceStartAddress: u32,
    #[doc = "< Total Size of Device."]
    pub deviceSize: u32,
    #[doc = "< Programming Page Size."]
    pub pageSize: u32,
    #[doc = "< Type number."]
    pub sectorsTypeNbr: u32,
    #[doc = "< Device sectors."]
    pub sectors: *mut deviceSector,
}

#[doc = " \\struct  externalStorageInfo\n \\brief   Get external storage informations useful for external Loader."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct externalStorageInfo {
    pub externalLoaderNbr: ::std::os::raw::c_uint,
    pub externalLoader: *mut externalLoader,
}

extern "C" {
    #[doc = " \\brief This routine allows to get ST-LINK conneted probe(s).\n \\param stLinkList  : Filled with the connected ST-LINK list and its default configurations.\n \\param shared      : Enable shared mode allowing connection of two or more instances to the same ST-LINK probe.\n \\return Number of the ST-LINK probes already exists.\n \\warning The Share option is useful only with ST-LINK Server.\n \\note  At the end of usage, #deleteInterfaceList must have been called."]
    pub fn getStLinkList(
        stLinkList: *mut *mut debugConnectParameters,
        shared: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to start connection to device through SWD or JTAG interfaces.\n \\param debugParameters : Indicates customized configuration for ST-LINK connection,\n It is recommended to check [debugConnectParameters] fields before connection.\n \\return 0 if the connection successfully established, otherwise an error occurred."]
    pub fn connectStLink(debugParameters: debugConnectParameters) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine used to apply a target reset.\n \\note  Reset operation is only available with JTAG/SWD debug interface.\n \\param rstMode : Indicates the reset type Soft/Hard/Core #debugResetMode. \\n\n \\return 0 if the reset operation finished successfully, otherwise an error occurred."]
    pub fn reset(rstMode: debugResetMode) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to get connected serial ports.\n \\param usartList : Receive serial ports list and its default configurations.\n \\return Number of serial ports already connected.\n \\note  At the end of usage, #deleteInterfaceList must have been called."]
    pub fn getUsartList(usartList: *mut *mut usartConnectParameters) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to start connection to device through USART interface.\n \\param usartParameters : Indicates customized configuration for USART connection.\n \\return 0 if the connection successfully established, otherwise an error occurred."]
    pub fn connectUsartBootloader(usartParameters: usartConnectParameters)
        -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to send a single byte through the USART interface.\n \\param byte : The data to be written\n \\return 0 if the sending operation correctly achieved, otherwise an error occurred."]
    pub fn sendByteUart(byte: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to get connected DFU devices.\n \\param dfuList : Receive DFU devices list and its default configurations.\n \\param iPID : Indicate the Product ID to be used for DFU interface.\n \\param iVID : Indicate the Vendor ID to be used for DFU interface.\n \\return Number of DFU devices already connected.\n \\note  At the end of usage, #deleteInterfaceList must have been called."]
    pub fn getDfuDeviceList(
        dfuList: *mut *mut dfuDeviceInfo,
        iPID: ::std::os::raw::c_int,
        iVID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to start a simple connection through USB DFU interface.\n \\param usbIndex : Indicates the index of DFU ports already connected.\n \\return 0 if the connection successfully established, otherwise an error occurred."]
    pub fn connectDfuBootloader(usbIndex: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to start connection to device through USB DFU interface.\n \\param dfuConnectParameters : Indicates the dfu connection parameters\n \\return 0 if the connection successfully established, otherwise an error occurred.\n \\note  It's recommanded to use this routine to disable readout protection when connecting a MCU based device."]
    pub fn connectDfuBootloader2(dfuParameters: dfuConnectParameters) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to start connection to device through SPI interface.\n \\param spiParameters : Indicates customized configuration for  SPI connection\n \\return 0 if the connection successfully established, otherwise an error occurred."]
    pub fn connectSpiBootloader(spiParameters: spiConnectParameters) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to start connection to device through CAN interface.\n \\param canParameters : Indicates customized configuration for  CAN connection\n \\return 0 if the connection successfully established, otherwise an error occurred.\n \\warning To have CAN full support, you must have St-Link firmware version at least v3JxMxB2."]
    pub fn connectCanBootloader(canParameters: canConnectParameters) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to start connection to device through I2C interface.\n \\param i2cParameters : Indicates customized configuration for  I2C connection\n \\return 0 if the connection successfully established, otherwise an error occurred."]
    pub fn connectI2cBootloader(i2cParameters: i2cConnectParameters) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to choose your custom display.\n \\param c : Fill the struct to customize the display tool.\n \\note This function must be called first of all to ensure the display management."]
    pub fn setDisplayCallbacks(c: displayCallBacks);
}
extern "C" {
    #[doc = " \\brief This routine allows to choose the verbosity level for display.\n \\param level : Indicates the verbosity number 0, 1 or 3."]
    pub fn setVerbosityLevel(level: ::std::os::raw::c_int);
}
extern "C" {
    #[doc = " \\brief This routine allows to check connection status [maintained or lost].\n \\return 1 if the device is already connected, otherwise the connection to device is lost."]
    pub fn checkDeviceConnection() -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to get general device informations.\n \\return Structure #GeneralInf in which the informations are stored."]
    pub fn getDeviceGeneralInf() -> *mut generalInf;
}
extern "C" {
    #[doc = " \\brief This routine allows to receive memory data on the used interface with the configration already initialized.\n \\param address   : The address to start reading from.\n \\param data      : Pointer to the data buffer.\n \\param size      : It indicates the size for read data.\n \\return 0 if the reading operation correctly finished, otherwise an error occurred.\n \\warning Unlike ST-LINK interface, the Bootloader interface can access only to some specific memory regions."]
    pub fn readMemory(
        address: ::std::os::raw::c_uint,
        data: *mut *mut ::std::os::raw::c_uchar,
        size: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to write memory data on the user interface with the configration already initialized.\n \\param address   : The address to start writing from.\n \\param data      : Pointer to the data buffer.\n \\param size      : It indicates the size for write data.\n \\return 0 if the writing operation correctly finished, otherwise an error occurred.\n \\warning Unlike ST-LINK interface, the Bootloader interface can access only to some specific memory regions."]
    pub fn writeMemory(
        address: ::std::os::raw::c_uint,
        data: *mut ::std::os::raw::c_char,
        size: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to write sector data on the user interface with the configration already initialized.\n \\param address   : The address to start writing from.\n \\param data      : Pointer to the data buffer.\n \\param size      : It indicates the size for write data.\n \\return 0 if the writing operation correctly finished, otherwise an error occurred.\n \\warning Unlike ST-LINK interface, the Bootloader interface can access only to some specific memory regions.\n \\warning Data size should not exceed sector size."]
    pub fn editSector(
        address: ::std::os::raw::c_uint,
        data: *mut ::std::os::raw::c_char,
        size: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to download data from a file to the memory.\n File formats that are supported : hex, bin, srec, tsv, elf, axf, out, stm32, ext\n \\param filePath  : Indicates the full path of the considered file.\n \\param address   : The address to start downloading from.\n \\param skipErase : In case to win in term time and if we have a blank device, we can skip erasing memory before programming [skipErase=0].\n \\param verify    : To add verification step after downloading.\n \\param binPath   : Path of the binary file.\n \\return 0 if the downloading operation correctly finished, otherwise an error occurred."]
    pub fn downloadFile(
        filePath: *const wchar_t,
        address: ::std::os::raw::c_uint,
        skipErase: ::std::os::raw::c_uint,
        verify: ::std::os::raw::c_uint,
        binPath: *const wchar_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to run the application.\n \\param address : The address to start executing from.\n In most cases, the program will run from the Flash memory starting from 0x08000000.\n \\return 0 if the execution correctly started, otherwise an error occurred."]
    pub fn execute(address: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to erase the whole Flash memory.\n \\return 0 if the operation finished successfully, otherwise an error was occurred.\n \\note Depending on the device, this routine can take a particular period of time."]
    pub fn massErase(sFlashMemName: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to erase specific sectors of the Flash memory.\n \\param sectors   : Indicates the indexs of the specific sectors to be erased.\n \\param sectorNbr : The number of chosen sectors.\n \\return 0 if the operation finished successfully, otherwise an error occurred.\n \\note Each circuit has a specific number of Flash memory sectors."]
    pub fn sectorErase(
        sectors: *mut ::std::os::raw::c_uint,
        sectorNbr: ::std::os::raw::c_uint,
        sFlashMemName: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to disable the readout protection.\n If the memory is not protected, a message appears to indicate that the device is not\n under Readout protection and the command has no effects.\n \\return 0 if the disabling correctly accomplished, otherwise an error occurred.\n \\note Depending on the device used, this routine take a specific time."]
    pub fn readUnprotect() -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows the TZEN Option Byte regression.\n \\return 0 if the disabling correctly accomplished, otherwise an error occurred.\n \\note Depending on the device used, this routine take a specific time."]
    pub fn tzenRegression() -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to know the interface what is in use.\n \\return The target interface type #targetInterfaceType, otherwise -1."]
    pub fn getTargetInterfaceType() -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to drop the current read/write operation.\n \\return 0 if there is no call for stop operation, otherwise 1."]
    pub fn getCancelPointer() -> *mut ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to open and get data from any supported file extension.\n \\param filePath : Indicates the full path of the considered file.\n \\return Pointer to #fileData_C if the file has hex, bin, srec or elf as extension, otherwise a null pointer to indicate that the file type is not supported."]
    pub fn fileOpen(filePath: *const wchar_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[doc = " \\brief This routine allows to clean up the handled file data.\n \\param data"]
    pub fn freeFileData(data: *mut fileData_C);
}
extern "C" {
    #[doc = " \\brief This routine allows to diplay the Option bytes"]
    pub fn obDisplay() -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to verfiy if the indicated file data is identical to Flash memory content.\n \\param fileData : Input file name.\n \\param address  : The address to start verifying from, it's considered only if the file has .bin or .binary as extension.\n \\return 0 if the file data matching Flash memory content, otherwise an error occurred or the data is mismatched."]
    pub fn verify(
        fileData: *mut fileData_C,
        address: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to save the data file content to another file.\n \\param fileData  : Input file name.\n \\param sFileName : Output file name.\n \\return 0 if the output file was created successfully, otherwise an error occurred."]
    pub fn saveFileToFile(
        fileData: *mut fileData_C,
        sFileName: *const wchar_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to save Flash memory content to file.\n \\param address   : The address to start saving from.\n \\param size      : Data size to be saved.\n \\param sFileName : Indicates the file name.\n \\return 0 if the data copy was acheived successfully, otherwise an error occurred.\n \\note The file name must finish with an extension \".hex\", \".bin\" or \".srec\""]
    pub fn saveMemoryToFile(
        address: ::std::os::raw::c_int,
        size: ::std::os::raw::c_int,
        sFileName: *const wchar_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to clean up and disconnect the current connected target.\n \\note This routine disconnect the target and delete the loaded Flash Loaders."]
    pub fn disconnect();
}
extern "C" {
    #[doc = " \\brief This routine allows to clear the list of each created interface.\n \\note The list is filled by #getStlinkList, #getDfuDeviceList or #getUsartList."]
    pub fn deleteInterfaceList();
}
extern "C" {
    #[doc = " \\brief This routine allows to enter and make an automatic process for memory management through JTAG/SWD, UART, DFU, SPI, CAN and I²C interfaces.\n \\param filePath      : Indicates the full file path.\n \\param address       : The address to start downloading from.\n \\param skipErase     : If we have a blank device, we can skip erasing memory before programming [skipErase=0].\n \\param verify        : Add verification step after downloading.\n \\param isMassErase   : Erase the whole Flash memory.\n \\param obCommand     : Indicates the option bytes commands to be loaded \"-ob [optionbyte=value] [optionbyte=value]...\"\n \\param run           : Start the application.\n \\warning Connection to target must be established before performing automatic mode."]
    pub fn automaticMode(
        filePath: *const wchar_t,
        address: ::std::os::raw::c_uint,
        skipErase: ::std::os::raw::c_uint,
        verify: ::std::os::raw::c_uint,
        isMassErase: ::std::os::raw::c_int,
        obCommand: *mut ::std::os::raw::c_char,
        run: ::std::os::raw::c_int,
    );
}
extern "C" {
    #[doc = " \\brief This routine allows to get Flash storage information.\n \\param deviceStorageStruct   : The data strcurure to load memory sectors information.\n \\return 0 if the operation was acheived successfully, otherwise an error occurred."]
    pub fn getStorageStructure(
        deviceStorageStruct: *mut *mut storageStructure,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows program the given Option Byte.\n The option bytes are configured by the end user depending on the application requirements.\n \\param command : Indicates the command to execute.\n \\return 0 if the programming Option Byte correctly executed, otherwise an error occurred.\n \\note The command must written as: -ob [optionbyte=value] [optionbyte=value] ...\n \\code\n int ob = sendOptionBytesCmd(\"–ob rdp=0x0 BOR_LEV=0\");\n \\endcode"]
    pub fn sendOptionBytesCmd(command: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to get option bytes values of the connected target.\n \\return Structure #Peripheral_C in which the option bytes descriptions are stored."]
    pub fn initOptionBytesInterface() -> *mut peripheral_C;
}
extern "C" {
    #[doc = " \\brief This routine allows to specify the location of Flash Loader.\n \\param path : Indicates the full path of the considered folder."]
    pub fn setLoadersPath(path: *const ::std::os::raw::c_char);
}
extern "C" {
    #[doc = " \\brief This routine allows to specify the path of the external Loaders to be loaded.\n \\param path : Indicates the full path of the folder containing external Loaders.\n \\param externalLoaderInfo : Structure in which the external Loaders informations are stored."]
    pub fn setExternalLoaderPath(
        path: *const ::std::os::raw::c_char,
        externalLoaderInfo: *mut *mut externalLoader,
    );
}
extern "C" {
    #[doc = " \\brief This routine allows to get available external Loaders in the mentioned path.\n \\param path : Indicates the full path containing ExternalLoader folder.\n \\param externalStorageNfo : Structure in which we get storage information.\n \\return 1 if the External loaders cannot be loaded from the path, otherwise 0.\n \\warning All external Loader files should have the extension \"stldr\"."]
    pub fn getExternalLoaders(
        path: *const ::std::os::raw::c_char,
        externalStorageNfo: *mut *mut externalStorageInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to unload an external Loaders.\n \\param path : Indicates the full path of the external Loader file ready for unloading."]
    pub fn removeExternalLoader(path: *const ::std::os::raw::c_char);
}
extern "C" {
    #[doc = " \\brief This routine allows to delete all target Flash Loaders."]
    pub fn deleteLoaders();
}
extern "C" {
    #[doc = " \\brief This routine allows to read the device unique identifier.\n \\param data : Pointer to the data buffer."]
    pub fn getUID64(data: *mut *mut ::std::os::raw::c_uchar) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to erase the BLE stack firmware.\n \\return 0 if the operation was acheived successfully, otherwise an error occurred."]
    pub fn firmwareDelete() -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to make upgrade of BLE stack firmware or FUS firmware.\n \\param filePath : Indicates the full path of the firmware to be programmed.\n \\param address : Start address of download.\n \\param firstInstall : 1 if it is the first installation, otherwise 0, to ignore the firmware delete operation.\n \\param startStack : Starts the stack after programming.\n \\param verify : Verify if the download operation is achieved successfully before starting the upgrade.\n \\return true if the operation was acheived successfully, otherwise an error occurred."]
    pub fn firmwareUpgrade(
        filePath: *const wchar_t,
        address: ::std::os::raw::c_uint,
        firstInstall: ::std::os::raw::c_uint,
        startStack: ::std::os::raw::c_uint,
        verify: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to start the programmed Stack.\n \\return true if the Stack was started successfully, otherwise an error occurred."]
    pub fn startWirelessStack() -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to start the programmed Stack.\n \\param filePath : Indicates the full path of the key file.\n \\note This is the public key generated by STM32TrustedPackageCreator when signing the firmware using -sign command.\n \\return true if the update was performed successfully, otherwise an error occurred."]
    pub fn updateAuthKey(filePath: *const wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to lock the authentication key and once locked, it is no longer possible to change it.\n \\return 0 if the lock step was performed successfully, otherwise an error occurred."]
    pub fn authKeyLock() -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to write a customized user key.\n \\param filePath : Indicates the full path of the key file.\n \\param keyType  : String indicating the key type to be used \"Simple\", \"Master\", \"Encrypted\".\n \\return 0 if the write was performed successfully, otherwise an error occurred."]
    pub fn writeUserKey(
        filePath: *const wchar_t,
        keyType: ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to activate the AntiRollBack.\n \\return true if the activation was done successfully, otherwise an error occurred."]
    pub fn antiRollBack() -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to start and establish a communication with the FUS operator.\n \\return true if the FUS operator was started successfully, otherwise an error occurred.\n \\note Availbale only for ST-LINK interfaces."]
    pub fn startFus() -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine allows to set default option Bytes."]
    pub fn unlockchip() -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " \\brief This routine aims to launch the Secure Secret Provisioning.\n \\param sspFile  : Indicates the full path of the ssp file [Use STM32TrustedPackageCreator to generate a ssp image].\n \\param licenseFile  : Indicates the full path of the license file. If you are trying to start the SSP without HSM, the hsmSlotId should be 0.\n \\param tfaFile  : Indicates the full path of the tfa-ssp file.\n \\param hsmSlotId  : Indicates the HSM slot ID.\n \\return 0 if the SSP was finished successfully, otherwise an error occurred.\n \\note If you are trying to start the SSP with HSM, the licenseFile parametre should be empty."]
    pub fn programSsp(
        sspFile: *const wchar_t,
        licenseFile: *const wchar_t,
        tfaFile: *const wchar_t,
        hsmSlotId: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " @brief getHsmFirmwareID: this routine aims to get the HSM Firmware Identifier.\n @param hsmSlotId: The slot index of the plugud-in HSM\n @return string that contains the HSM Firmware Identifier."]
    pub fn getHsmFirmwareID(hsmSlotId: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " @brief getHsmCounter: this routine aims to get the current HSM counter.\n @param hsmSlotId: The slot index of the plugud-in HSM\n @return Counter value"]
    pub fn getHsmCounter(hsmSlotId: ::std::os::raw::c_int) -> ::std::os::raw::c_ulong;
}
extern "C" {
    #[doc = " @brief getHsmState: this routine aims to get the HSM State.\n @param hsmSlotId: The slot index of the plugud-in HSM\n @return string with possible values: ST_STATE , OEM_STATE, OPERATIONAL_STATE , UNKNOWN_STATE"]
    pub fn getHsmState(hsmSlotId: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " @brief getHsmVersion: this routine aims to get the HSM version.\n @param hsmSlotId: The slot index of the plugud-in HSM\n @return string with possible values: 1 , 2"]
    pub fn getHsmVersion(hsmSlotId: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " @brief getHsmType: this routine aims to get the HSM type.\n @param hsmSlotId: The slot index of the plugud-in HSM\n @return string with possible values: SFI. SMU. SSP..."]
    pub fn getHsmType(hsmSlotId: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " @brief getHsmLicense: this routine aims to get and save the HSM license into a binary file.\n @param hsmSlotId: The slot index of the plugud-in HSM\n @param outLicensePath: path of the output binary file.\n @return 0 if the operation was finished successfully, otherwise an error occurred.\n \\note Connection to target must be established before performing this routine."]
    pub fn getHsmLicense(
        hsmSlotId: ::std::os::raw::c_int,
        outLicensePath: *const wchar_t,
    ) -> ::std::os::raw::c_int;
}
