use pcan_basic_sys as pcan;
use std::convert::From;
use std::io;


#[derive(Debug, Clone, Copy)]
pub enum BusStatus {
    Unknown,
    BusLight, // Bus error: an error counter reached the 'light' limit
    BusHeavy, // Bus error: an error counter reached the 'heavy' limit
    BusWarning, // Bus error: an error counter reached the 'warning' limit
    BusPassive, // Bus error: the CAN controller is error passive
    BusOff, // Bus error: the CAN controller is in bus-off state
}

impl From<u32> for BusStatus {
    fn from(x: u32) -> BusStatus {
        match x {
            pcan::PCAN_ERROR_BUSLIGHT => BusStatus::BusLight,
            pcan::PCAN_ERROR_BUSHEAVY => BusStatus::BusHeavy,
            pcan::PCAN_ERROR_BUSPASSIVE => BusStatus::BusPassive,
            pcan::PCAN_ERROR_BUSOFF => BusStatus::BusOff,
            _ => BusStatus::Unknown,
        }
    }
}

impl From<BusStatus> for u32 {
    fn from(x: BusStatus) -> u32 {
        match x {
            BusStatus::BusLight => pcan::PCAN_ERROR_BUSLIGHT,
            BusStatus::BusHeavy => pcan::PCAN_ERROR_BUSHEAVY,
            BusStatus::BusWarning => pcan::PCAN_ERROR_BUSWARNING,
            BusStatus::BusPassive => pcan::PCAN_ERROR_BUSPASSIVE,
            BusStatus::BusOff => pcan::PCAN_ERROR_BUSOFF,
            BusStatus::Unknown => 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum QueueStatus {
    Unknown,
    RecieveEmpty, // Receive queue is empty
    Overrun, // Receive queue was read too late
    TransmitFull, // Transmit queue is full
}

impl From<u32> for QueueStatus {
    fn from(x: u32) -> QueueStatus {
        match x {
            pcan::PCAN_ERROR_QRCVEMPTY => QueueStatus::RecieveEmpty,
            pcan::PCAN_ERROR_QOVERRUN => QueueStatus::Overrun,
            pcan::PCAN_ERROR_QXMTFULL => QueueStatus::TransmitFull,
            _ => QueueStatus::Unknown,
        }
    }
}

impl From<QueueStatus> for u32 {
    fn from(x: QueueStatus) -> u32 {
        match x {
            QueueStatus::RecieveEmpty => pcan::PCAN_ERROR_QRCVEMPTY,
            QueueStatus::Overrun => pcan::PCAN_ERROR_QOVERRUN,
            QueueStatus::TransmitFull => pcan::PCAN_ERROR_QXMTFULL,
            QueueStatus::Unknown => 0,
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub enum HandleStatus {
    Unknown,
    InvalidHwHandle,
    InvalidNetworkHandle,
    InvalidClientHandle,
}

impl From<u32> for HandleStatus {
    fn from(x: u32) -> HandleStatus {
        match x {
            pcan::PCAN_ERROR_ILLHW => HandleStatus::InvalidHwHandle,
            pcan::PCAN_ERROR_ILLNET => HandleStatus::InvalidNetworkHandle,
            pcan::PCAN_ERROR_ILLCLIENT => HandleStatus::InvalidClientHandle,
            _ => HandleStatus::Unknown,
        }
    }
}

impl From<HandleStatus> for u32 {
    fn from(x: HandleStatus) -> u32 {
        match x {
            HandleStatus::InvalidHwHandle => pcan::PCAN_ERROR_ILLHW,
            HandleStatus::InvalidNetworkHandle => pcan::PCAN_ERROR_ILLNET,
            HandleStatus::InvalidClientHandle => pcan::PCAN_ERROR_ILLCLIENT,
            HandleStatus::Unknown => 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PCANStatus {
    Ok, // No Error
    XmtFull, // Transmit buffer in CAN controller is full
    Overrun, // CAN controller was read too late
    Bus(BusStatus),
    Queue(QueueStatus),
    RegTest, // Test of the CAN controller hardware registers failed (no hardware found)
    NoDriver, // Driver not loaded
    HardwareInUse, // Hardware already in use by a Net
    NetworkInUse, // A Client is already connected to the Net
    Handle(HandleStatus),
    ResourceCannotBeCreated,
    InvalidParameter,
    InvalidParameterValue,
    Unknown,
    InvalidData,
    Caution,
    NotInitialized,
    InvalidOperation,
}

impl From<PCANStatus> for u32 {
    fn from(status: PCANStatus) -> u32 {
        match status {
            PCANStatus::Ok => pcan::PCAN_ERROR_OK, 
            PCANStatus::XmtFull => pcan::PCAN_ERROR_XMTFULL, 
            PCANStatus::Overrun => pcan::PCAN_ERROR_OVERRUN, 
            PCANStatus::Bus(x) => u32::from(x), 
            PCANStatus::Queue(x) => u32::from(x), 
            PCANStatus::RegTest => pcan::PCAN_ERROR_REGTEST, 
            PCANStatus::NoDriver => pcan::PCAN_ERROR_NODRIVER, 
            PCANStatus::HardwareInUse => pcan::PCAN_ERROR_HWINUSE, 
            PCANStatus::NetworkInUse => pcan::PCAN_ERROR_NETINUSE, 
            PCANStatus::Handle(x) => u32::from(x), 
            PCANStatus::ResourceCannotBeCreated => pcan::PCAN_ERROR_RESOURCE, 
            PCANStatus::InvalidParameter => pcan::PCAN_ERROR_ILLPARAMTYPE, 
            PCANStatus::InvalidParameterValue => pcan::PCAN_ERROR_ILLPARAMVAL, 
            PCANStatus::Unknown => pcan::PCAN_ERROR_UNKNOWN, 
            PCANStatus::InvalidData => pcan::PCAN_ERROR_ILLDATA, 
            PCANStatus::Caution => pcan::PCAN_ERROR_CAUTION, 
            PCANStatus::NotInitialized => pcan::PCAN_ERROR_INITIALIZE, 
            PCANStatus::InvalidOperation => pcan::PCAN_ERROR_ILLOPERATION,             
        }
    }
}

impl From<u32> for PCANStatus {
    fn from(status: u32) -> PCANStatus {
        match status {        
            pcan::PCAN_ERROR_OK => PCANStatus::Ok,
            pcan::PCAN_ERROR_XMTFULL => PCANStatus::XmtFull,
            pcan::PCAN_ERROR_OVERRUN => PCANStatus::Overrun,  
            pcan::PCAN_ERROR_REGTEST => PCANStatus::RegTest,
            pcan::PCAN_ERROR_NODRIVER => PCANStatus::NoDriver,
            pcan::PCAN_ERROR_HWINUSE => PCANStatus::HardwareInUse,
            pcan::PCAN_ERROR_NETINUSE => PCANStatus::NetworkInUse,
            pcan::PCAN_ERROR_RESOURCE => PCANStatus::ResourceCannotBeCreated, 
            pcan::PCAN_ERROR_ILLPARAMTYPE => PCANStatus::InvalidParameter, 
            pcan::PCAN_ERROR_ILLPARAMVAL => PCANStatus::InvalidParameterValue, 
            pcan::PCAN_ERROR_UNKNOWN => PCANStatus::Unknown, 
            pcan::PCAN_ERROR_ILLDATA => PCANStatus::InvalidData, 
            pcan::PCAN_ERROR_CAUTION => PCANStatus::Caution, 
            pcan::PCAN_ERROR_INITIALIZE => PCANStatus::NotInitialized, 
            pcan::PCAN_ERROR_ILLOPERATION => PCANStatus::InvalidOperation,  
            x @ pcan::PCAN_ERROR_BUSLIGHT |
            x @ pcan::PCAN_ERROR_BUSHEAVY |
            x @ pcan::PCAN_ERROR_BUSPASSIVE |
            x @ pcan::PCAN_ERROR_BUSOFF => PCANStatus::Bus(BusStatus::from(x)),
            x @ pcan::PCAN_ERROR_QRCVEMPTY |
            x @ pcan::PCAN_ERROR_QOVERRUN |
            x @ pcan::PCAN_ERROR_QXMTFULL => PCANStatus::Queue(QueueStatus::from(x),),
            x @ pcan::PCAN_ERROR_ILLHW |
            x @ pcan::PCAN_ERROR_ILLNET |
            x @ pcan::PCAN_ERROR_ILLCLIENT => PCANStatus::Handle(HandleStatus::from(x),),
            _ => PCANStatus::Unknown,
        }
    }
}

error_chain! {
    foreign_links {
        Io(io::Error) #[doc = "Error during IO"];
    }   
 
    errors {
        PCAN(status: PCANStatus) {
            description("An error occured while communicating with the PCAN Device")
            display("PCAN Device code: 0x{:08x}", u32::from(*status))
        }

        CANFrame {
            description("Invalid can frame")
            display("Could not make can frame")
        }
    }
}

pub fn pcan_fn_to_result(return_code: u32) -> Result<()> {
    match PCANStatus::from(return_code) {
        PCANStatus::Ok => Ok(()),
        x @ _ => Err(ErrorKind::PCAN(x).into()),
    }
}
