//! PCAN Basic (No FD support)
//!
extern crate pcan_basic_sys;
#[macro_use]
extern crate error_chain;

use pcan_basic_sys as pcan;
pub mod types;
pub mod errors;
pub use errors::*;
pub use types::*;

/// A Handle for a PCAN Device.
#[derive(Debug)]
pub struct PCANDevice {
    fd: Handle,
}

#[derive(Debug)]
pub struct CANFrame(pub pcan::TPCANMsg);

impl CANFrame {
    pub fn new(id: u32, data: &[u8], rtr: bool) -> Result<CANFrame> {
        if data.len() > 8 {
            bail!(ErrorKind::CANFrame);
        }

        let msg_type = if rtr {
            MessageType::Rtr
        } else {
            MessageType::Standard
        };

        let mut frame = CANFrame(
            pcan::TPCANMsg {
                ID: id,
                MSGTYPE: u8::from(msg_type),
                LEN: data.len() as u8,
                DATA: [0; 8],
            },
        );

        frame.0.DATA.clone_from_slice(data);

        Ok(frame)
    }

    #[inline(always)]
    pub fn id(&self) -> u32 {
        self.0.ID
    }

    #[inline(always)]
    pub fn set_id(&mut self, id: &u32) {
        self.0.ID = *id;
    }

    #[inline(always)]
    pub fn msg_type(&self) -> MessageType {
        MessageType::from(self.0.MSGTYPE)
    }

    #[inline(always)]
    pub fn set_msg_type(&mut self, msg_type: &MessageType) {
        self.0.MSGTYPE = u8::from(*msg_type)
    }

    #[inline(always)]
    pub fn len(&self) -> u8 {
        self.0.LEN
    }

    #[inline(always)]
    pub fn data(&self) -> &[u8] {
        self.0.DATA.as_ref()
    }

    #[inline(always)]
    pub fn mut_data(&mut self) -> &mut [u8] {
        self.0.DATA.as_mut()
    }
}



impl PCANDevice {
    /// Open a PCAN Device
    pub fn open(handle: &Handle, baudrate: &BaudRate) -> Result<PCANDevice> {
        let device = PCANDevice { fd: *handle };
        let handle_value = device.fd.to_value().expect("Invalid handle");

        let err = unsafe {
            pcan::CAN_Initialize(handle_value, baudrate.to_value(), 0, 0, 0)
        };

        pcan_fn_to_result(err)?;

        Ok(device)
    }

    pub fn read_frame(&self) -> Result<(CANFrame, pcan::TPCANTimestamp)> {
        let mut frame: pcan::TPCANMsg = pcan::TPCANMsg {
            ID: 0,
            MSGTYPE: 0,
            LEN: 0,
            DATA: [0; 8],
        };

        let mut timestamp: pcan::TPCANTimestamp = pcan::TPCANTimestamp {
            micros: 0,
            millis: 0,
            millis_overflow: 0,
        };

        let err = unsafe {
            pcan::CAN_Read(
                self.fd.to_value().unwrap() as u16,
                &mut frame as *mut pcan::TPCANMsg,
                &mut timestamp as *mut pcan::TPCANTimestamp,
            )
        };

        pcan_fn_to_result(err)?;

        Ok((CANFrame(frame), timestamp))
    }

    pub fn write_frame(&self, frame: &mut CANFrame) -> Result<()> {
        let err = unsafe {
            pcan::CAN_Write(
                self.fd.to_value().unwrap() as u16,
                &mut (frame.0) as *mut pcan::TPCANMsg,
            )
        };

        pcan_fn_to_result(err)
    }

    pub fn close(&mut self) -> Result<()> {
        let err = unsafe {
            pcan::CAN_Uninitialize(self.fd.to_value().unwrap() as u16)
        };

        pcan_fn_to_result(err)
    }
}

impl Drop for PCANDevice {
    fn drop(&mut self) {
        self.close().ok();
    }
}
