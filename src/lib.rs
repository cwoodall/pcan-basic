//! PCAN Basic (No FD support)
//!
extern crate pcan_basic_bindings;

use pcan_basic_bindings as pcan;
pub mod types;
pub mod errors;
pub use types::{BaudRate, Handle, Device, ParameterValue, MessageType,
                LogParameterValue, Mode, Parameter, PCANParameterValue,
                ServiceParameterValue, TraceParameterValue, Type};

/// A Handle for a PCAN Device.
#[derive(Debug)]
pub struct PCANDevice {
    fd: Handle,
}

#[derive(Debug)]
pub struct CANFrame(pub pcan::TPCANMsg);

impl CANFrame {
    pub fn new(
        id: u32,
        data: &[u8],
        rtr: bool,
        err: bool,
    ) -> Result<CANFrame, errors::Error> {
        if data.len() > 8 {
            return Err(errors::Error::Unknown);
        }

        let msg_type = if rtr {
            MessageType::Rtr
        } else {
            MessageType::Standard
        };

        let mut frame = CANFrame(
            pcan::TPCANMsg {
                ID: id,
                MSGTYPE: msg_type.to_value() as u8,
                LEN: data.len() as u8,
                DATA: [0; 8],
            },
        );

        frame.0.DATA.clone_from_slice(data);

        Ok(frame)
    }
}


impl PCANDevice {
    /// Open a PCAN Device
    pub fn open(
        handle: &Handle,
        baudrate: &BaudRate,
    ) -> Result<PCANDevice, errors::Error> {
        let mut device = PCANDevice { fd: *handle };
        let handle_value = device.fd.to_value().unwrap();

        // TODO(cw) actual error handling
        let err = unsafe {
            pcan::CAN_Initialize(handle_value, baudrate.to_value(), 0, 0, 0);
        };

        Ok(device)
    }

    pub fn close(&mut self) -> Result<(), errors::Error> {
        // TODO(cw) actual error handling
        let err = unsafe {
            pcan::CAN_Uninitialize(self.fd.to_value().unwrap() as u16)
        };
        Ok(())
    }
}

impl Drop for PCANDevice {
    fn drop(&mut self) {
        self.close().ok();
    }
}
