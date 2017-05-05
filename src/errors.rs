pub enum BusError {
    BusLight, // Bus error: an error counter reached the 'light' limit
    BusHeavy, // Bus error: an error counter reached the 'heavy' limit
    BusWarning, // Bus error: an error counter reached the 'warning' limit
    BusPassive, // Bus error: the CAN controller is error passive
    BusOff, // Bus error: the CAN controller is in bus-off state
}

pub enum QueueError {
    RecieveEmpty, // Receive queue is empty
    OverRun, // Receive queue was read too late
    TransmitFull, // Transmit queue is full
}

pub enum HandleError {
    InvalidHwHandle,
    InvalidNetworkHandle,
    InvalidClientHandle,
}

pub enum Error {
    Ok, // No Error
    XmtFull, // Transmit buffer in CAN controller is full
    Overrun, // CAN controller was read too late
    BusError(BusError),
    QueueError(QueueError),
    RegTest, // Test of the CAN controller hardware registers failed (no hardware found)
    NoDriver, // Driver not loaded
    HardwareInUse, // Hardware already in use by a Net
    NetworkInUse, // A Client is already connected to the Net
    HandleError(HandleError),
    ResourceCannotBeCreated,
    InvalidParameter,
    InvalidParameterValue,
    Unknown,
    InvalidData,
    Caution,
    NotInitialized,
    InvalidOperation,
}
