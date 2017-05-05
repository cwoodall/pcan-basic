use pcan_basic_bindings as pcan;

/// Baud rate codes = BTR0/BTR1 register values for the CAN controller.
#[derive(Debug, Clone, Copy)]
pub enum BaudRate {
    Baud1M, //   1 MBit/s
    Baud800K, // 800 kBit/s
    Baud500K, // 500 kBit/s
    Baud250K, // 250 kBit/s
    Baud125K, // 125 kBit/s
    Baud100K, // 100 kBit/s
    Baud95K, //  95,238 kBit/s
    Baud83K, //  83,333 kBit/s
    Baud50K, //  50 kBit/s
    Baud47K, //  47,619 kBit/s
    Baud33K, //  33,333 kBit/s
    Baud20K, //  20 kBit/s
    Baud10K, //  10 kBit/s
    Baud5K, //   5 kBit/s
}

impl BaudRate {
    /// Convert the BaudRate enum into a value of the BTR0/BTR1 register as
    /// defined in the PCAN C bindings.
    pub fn to_value(&self) -> u16 {
        match *self {
            BaudRate::Baud1M => pcan::PCAN_BAUD_1M as u16,
            BaudRate::Baud800K => pcan::PCAN_BAUD_800K as u16,
            BaudRate::Baud500K => pcan::PCAN_BAUD_500K as u16,
            BaudRate::Baud250K => pcan::PCAN_BAUD_250K as u16,
            BaudRate::Baud125K => pcan::PCAN_BAUD_125K as u16,
            BaudRate::Baud100K => pcan::PCAN_BAUD_100K as u16,
            BaudRate::Baud95K => pcan::PCAN_BAUD_95K as u16,
            BaudRate::Baud83K => pcan::PCAN_BAUD_83K as u16,
            BaudRate::Baud50K => pcan::PCAN_BAUD_50K as u16,
            BaudRate::Baud47K => pcan::PCAN_BAUD_47K as u16,
            BaudRate::Baud33K => pcan::PCAN_BAUD_33K as u16,
            BaudRate::Baud20K => pcan::PCAN_BAUD_20K as u16,
            BaudRate::Baud10K => pcan::PCAN_BAUD_10K as u16,
            BaudRate::Baud5K => pcan::PCAN_BAUD_5K as u16,
        }
    }
}

/// The handle/channel which we want the PCAN Basic library to connect to/use
#[derive(Debug, Clone, Copy)]
pub enum Handle {
    Undefined, // The undefined and default value of the handle
    Isa(u32), // ISA Bus Interface (1 - 8)
    Dng(u32), // Dng Bus Interface (1)
    Pci(u32), // PCI Card Interface (1-16)
    Usb(u32), // PCAN USB Interface Channels (1 to 16)
    Pcc(u32), // PCAN PC Card Interface (1 and 2)
    Lan(u32), // PCAN LAN Interface (1 to 16)
}

impl Handle {
    /// Get the value of the handle, this following mapping was derived from the
    /// pcan-basic C bindings.
    pub fn to_value(&self) -> Option<u16> {
        match *self {
            Handle::Undefined => Some(0 as u16),
            Handle::Isa(id @ 1...8) => Some((0x20 + id) as u16),
            Handle::Dng(1) => Some(0x31 as u16),
            Handle::Pci(id @ 1...8) => Some((0x40 + id) as u16),
            Handle::Pci(id @ 9...16) => Some((0x400 + id) as u16),
            Handle::Usb(id @ 1...8) => Some((0x50 + id) as u16),
            Handle::Usb(id @ 9...16) => Some((0x500 + id) as u16),
            Handle::Pcc(id @ 1...2) => Some((0x60 + id) as u16),
            Handle::Lan(id @ 1...16) => Some((0x800 + id) as u16),
            _ => None,
        }
    }
}

/// PCAN Devices (Removed those which do not have PCAN Basic support)
#[derive(Debug, Clone, Copy)]
pub enum Device {
    Undefined,
    Isa,
    Dng,
    Pci,
    Usb,
    Pcc,
    Lan,
}

impl Device {
    pub fn to_value(&self) -> u32 {
        match *self {
            Device::Isa => pcan::PCAN_ISA,
            Device::Dng => pcan::PCAN_DNG,
            Device::Pci => pcan::PCAN_PCI,
            Device::Usb => pcan::PCAN_USB,
            Device::Pcc => pcan::PCAN_PCC,
            Device::Lan => pcan::PCAN_LAN,
            _ => pcan::PCAN_NONE,
        }
    }
}

/// Parameters for IOCTL like controls of Device Driver Settings
#[derive(Debug, Clone, Copy)]
pub enum Parameter {
    DeviceNumber, // PCAN-USB device number parameter
    Power5Volts, // PCAN-PC Card 5-Volt power parameter
    ReceiveEvent, // PCAN receive event handler parameter
    MessageFilter, // PCAN message filter parameter
    ApiVersion, // PCAN-Basic API version parameter
    ChannelVersion, // PCAN device channel version parameter
    BusOffAutoReset, // PCAN Reset-On-Busoff parameter
    ListenOnly, // PCAN Listen-Only parameter
    LogLocation, // Directory path for log files
    LogStatus, // Debug-Log activation status
    LogConfigure, // Configuration of the debugged information (LOG_FUNCTION_***)
    LogText, // Custom insertion of text into the log file
    ChannelCondition, // Availability status of a PCAN-Channel
    HardwareName, // PCAN hardware name parameter
    ReceiveStatus, // Message reception status of a PCAN-Channel
    ControllerNumber, // CAN-Controller number of a PCAN-Channel
    TraceLocation, // Directory path for PCAN trace files
    TraceStatus, // CAN tracing activation status
    TraceSize, // Configuration of the maximum file size of a CAN trace
    TraceConfigure, // Configuration of the trace file storing mode (TRACE_FILE_***)
    ChannelIdentifying, // Physical identification of a USB based PCAN-Channel by blinking its associated LED
    ChannelFeatures, // Capabilities of a PCAN device (FEATURE_***)
    BitRateAdapting, // Using of an existing bit rate (PCAN-View connected to a channel)
    BitRateInfo, // Configured bit rate as Btr0Btr1 value
    BitRateInfoFd, // Configured bit rate as TPCANBitrateFD string
    BusSpeedNominal, // Configured nominal CAN Bus speed as Bits per seconds
    BusSpeedData, // Configured CAN data speed as Bits per seconds
    IpAddress, // Remote address of a LAN channel as string in IPv4 format
    LanServiceStatus, // Status of the Virtual PCAN-Gateway Service
}

impl Parameter {
    pub fn to_value(&self) -> u32 {
        match *self {
            Parameter::DeviceNumber => pcan::PCAN_DEVICE_NUMBER,
            Parameter::Power5Volts => pcan::PCAN_5VOLTS_POWER,
            Parameter::ReceiveEvent => pcan::PCAN_RECEIVE_EVENT,
            Parameter::MessageFilter => pcan::PCAN_MESSAGE_FILTER,
            Parameter::ApiVersion => pcan::PCAN_API_VERSION,
            Parameter::ChannelVersion => pcan::PCAN_CHANNEL_VERSION,
            Parameter::BusOffAutoReset => pcan::PCAN_BUSOFF_AUTORESET,
            Parameter::ListenOnly => pcan::PCAN_LISTEN_ONLY,
            Parameter::LogLocation => pcan::PCAN_LOG_LOCATION,
            Parameter::LogStatus => pcan::PCAN_LOG_STATUS,
            Parameter::LogConfigure => pcan::PCAN_LOG_CONFIGURE,
            Parameter::LogText => pcan::PCAN_LOG_TEXT,
            Parameter::ChannelCondition => pcan::PCAN_CHANNEL_CONDITION,
            Parameter::HardwareName => pcan::PCAN_HARDWARE_NAME,
            Parameter::ReceiveStatus => pcan::PCAN_RECEIVE_STATUS,
            Parameter::ControllerNumber => pcan::PCAN_CONTROLLER_NUMBER,
            Parameter::TraceLocation => pcan::PCAN_TRACE_LOCATION,
            Parameter::TraceStatus => pcan::PCAN_TRACE_STATUS,
            Parameter::TraceSize => pcan::PCAN_TRACE_SIZE,
            Parameter::TraceConfigure => pcan::PCAN_TRACE_CONFIGURE,
            Parameter::ChannelIdentifying => pcan::PCAN_CHANNEL_IDENTIFYING,
            Parameter::ChannelFeatures => pcan::PCAN_CHANNEL_FEATURES,
            Parameter::BitRateAdapting => pcan::PCAN_BITRATE_ADAPTING,
            Parameter::BitRateInfo => pcan::PCAN_BITRATE_INFO,
            Parameter::BitRateInfoFd => pcan::PCAN_BITRATE_INFO_FD,
            Parameter::BusSpeedNominal => pcan::PCAN_BUSSPEED_NOMINAL,
            Parameter::BusSpeedData => pcan::PCAN_BUSSPEED_DATA,
            Parameter::IpAddress => pcan::PCAN_IP_ADDRESS,
            Parameter::LanServiceStatus => pcan::PCAN_LAN_SERVICE_STATUS,
        }
    }
}

#[derive(Debug,Clone,Copy)]
pub enum PCANParameterValue {
    Off, // The PCAN parameter is not set (inactive)
    On, // The PCAN parameter is set (active)
    FilterClose, // The PCAN filter is closed. No messages will be received
    FilterOpen, // The PCAN filter is fully opened. All messages will be received
    FilterCustom, // The PCAN filter is custom configured. Only registered messages will be received
    ChannelUnavailable, // The PCAN-Channel handle is illegal, or its associated hardware is not available
    ChannelAvailable, // The PCAN-Channel handle is available to be connected (Plug&Play Hardware: it means furthermore that the hardware is plugged-in)
    ChannelOccupied, // The PCAN-Channel handle is valid, and is already being used
}

impl PCANParameterValue {
    pub fn to_value(&self) -> u32 {
        match *self {
            PCANParameterValue::Off => pcan::PCAN_PARAMETER_OFF,
            PCANParameterValue::On => pcan::PCAN_PARAMETER_ON,
            PCANParameterValue::FilterClose => pcan::PCAN_FILTER_CLOSE,
            PCANParameterValue::FilterOpen => pcan::PCAN_FILTER_OPEN,
            PCANParameterValue::FilterCustom => pcan::PCAN_FILTER_CUSTOM,
            PCANParameterValue::ChannelUnavailable => {
                pcan::PCAN_CHANNEL_UNAVAILABLE
            }
            PCANParameterValue::ChannelAvailable => {
                pcan::PCAN_CHANNEL_AVAILABLE
            }
            PCANParameterValue::ChannelOccupied => pcan::PCAN_CHANNEL_OCCUPIED,
        }
    }
}

#[derive(Debug,Clone,Copy)]
pub enum LogParameterValue {
    Default, // Logs system exceptions / errors
    Entry, // Logs the entries to the PCAN-Basic API functions
    Parameters, // Logs the parameters passed to the PCAN-Basic API functions
    Leave, // Logs the exits from the PCAN-Basic API functions
    Write, // Logs the CAN messages passed to the CAN_Write function
    Read, // Logs the CAN messages received within the CAN_Read function
}

impl LogParameterValue {
    pub fn to_value(&self) -> u32 {
        match *self {
            LogParameterValue::Default => pcan::LOG_FUNCTION_DEFAULT,
            LogParameterValue::Entry => pcan::LOG_FUNCTION_ENTRY,
            LogParameterValue::Parameters => pcan::LOG_FUNCTION_PARAMETERS,
            LogParameterValue::Leave => pcan::LOG_FUNCTION_LEAVE,
            LogParameterValue::Write => pcan::LOG_FUNCTION_WRITE,
            LogParameterValue::Read => pcan::LOG_FUNCTION_READ,
        }
    }
}

#[derive(Debug,Clone,Copy)]
pub enum TraceParameterValue {
    Single, // A single file is written until it size reaches PAN_TRACE_SIZE
    Segmented, // Traced data is distributed in several files with size PAN_TRACE_SIZE
    Date, // Includes the date into the name of the trace file
    Time, // Includes the start time into the name of the trace file
    Overwrite, // Causes the overwriting of available traces (same name)
}

impl TraceParameterValue {
    pub fn to_value(&self) -> u32 {
        match *self {
            TraceParameterValue::Single => pcan::TRACE_FILE_SINGLE,
            TraceParameterValue::Segmented => pcan::TRACE_FILE_SEGMENTED,
            TraceParameterValue::Date => pcan::TRACE_FILE_DATE,
            TraceParameterValue::Time => pcan::TRACE_FILE_TIME,
            TraceParameterValue::Overwrite => pcan::TRACE_FILE_OVERWRITE,
        }
    }
}

#[derive(Debug,Clone,Copy)]
pub enum ServiceParameterValue {
    Stopped, // The service is not running
    Running, // The service is running
}

impl ServiceParameterValue {
    pub fn to_value(&self) -> u32 {
        match *self {
            ServiceParameterValue::Stopped => pcan::SERVICE_STATUS_STOPPED,
            ServiceParameterValue::Running => pcan::SERVICE_STATUS_RUNNING,
        }
    }
}

#[derive(Debug,Clone,Copy)]
pub enum ParameterValue {
    PCAN(PCANParameterValue),
    Log(LogParameterValue),
    Trace(TraceParameterValue),
    FdCapable,
    Service(ServiceParameterValue),
}

impl ParameterValue {
    pub fn to_value(&self) -> u32 {
        match *self {
            ParameterValue::FdCapable => pcan::FEATURE_FD_CAPABLE,
            _ => self.to_value(),
        }
    }
}

/// PCAN Message Types
#[derive(Debug,Clone,Copy)]
pub enum MessageType {
    Standard,
    Rtr,
    Extended,
    Fd,
    Brs,
    Esi,
    Status,
}

impl MessageType {
    pub fn to_value(&self) -> u32 {
        match *self {
            MessageType::Standard => pcan::PCAN_MESSAGE_STANDARD,
            MessageType::Rtr => pcan::PCAN_MESSAGE_RTR,
            MessageType::Extended => pcan::PCAN_MESSAGE_EXTENDED,
            MessageType::Fd => pcan::PCAN_MESSAGE_FD,
            MessageType::Brs => pcan::PCAN_MESSAGE_BRS,
            MessageType::Esi => pcan::PCAN_MESSAGE_ESI,
            MessageType::Status => pcan::PCAN_MESSAGE_STATUS,
        }
    }
}

#[derive(Debug,Clone,Copy)]
pub enum Mode {
    Standard,
    Extended,
}

impl Mode {
    pub fn to_value(&self) -> u32 {
        match *self {
            Mode::Standard => pcan::PCAN_MODE_STANDARD,
            Mode::Extended => pcan::PCAN_MODE_EXTENDED,
        }
    }
}

#[derive(Debug,Clone,Copy)]
pub enum Type {
    Isa,
    IsaSja,
    IsaPhytec,
    Dng,
    DngEpp,
    DngSja,
    DngSjaEpp,
}

impl Type {
    pub fn to_value(&self) -> u32 {
        match *self {
            Type::Isa => pcan::PCAN_TYPE_ISA,
            Type::IsaSja => pcan::PCAN_TYPE_ISA_SJA,
            Type::IsaPhytec => pcan::PCAN_TYPE_ISA_PHYTEC,
            Type::Dng => pcan::PCAN_TYPE_DNG,
            Type::DngEpp => pcan::PCAN_TYPE_DNG_EPP,
            Type::DngSja => pcan::PCAN_TYPE_DNG_SJA,
            Type::DngSjaEpp => pcan::PCAN_TYPE_DNG_SJA_EPP,
        }
    }
}

#[cfg(test)]
mod tests {
    use {Handle, Parameter};
    use pcan_basic_bindings as pcan;
    #[test]
    fn it_works() {}

    #[test]
    fn handle_to_value() {
        assert!(Handle::Undefined.to_value() == Some(pcan::PCAN_NONEBUS));
        assert!(Handle::Isa(0).to_value() == None);
        assert!(Handle::Isa(1).to_value() == Some(pcan::PCAN_ISABUS1));
        assert!(Handle::Isa(8).to_value() == Some(pcan::PCAN_ISABUS8));
        assert!(Handle::Isa(9).to_value() == None);
        assert!(Handle::Usb(0).to_value() == None);
        assert!(Handle::Usb(1).to_value() == Some(pcan::PCAN_USBBUS1));
        assert!(Handle::Usb(8).to_value() == Some(pcan::PCAN_USBBUS8));
        assert!(Handle::Usb(9).to_value() == Some(pcan::PCAN_USBBUS9));
        assert!(Handle::Usb(16).to_value() == Some(pcan::PCAN_USBBUS16));
        assert!(Handle::Usb(17).to_value() == None);
    }
}
