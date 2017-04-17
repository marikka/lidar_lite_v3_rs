extern crate i2cdev;
use i2cdev::core::I2CDevice;

extern crate byteorder;
use byteorder::{ByteOrder, LittleEndian};

#[macro_use]
extern crate bitflags;

pub const LIDAR_LITE_DEFAULT_I2C_ADDRESS: u16 = 0x62;

const ACQ_COMMAND: u8 = 0x00;
const STATUS: u8 = 0x01;
const UNIT_ID_HIGH: u8 = 0x16;
const UNIT_ID_LOW: u8 = 0x17;
const FULL_DELAY_HIGH: u8 = 0x0f;
const FULL_DELAY_LOW: u8 = 0x10;
const RESET: u8 = 0x00;
const DISTANCE_WITHOUT_RECEIVER_BIAS_CORRECTION: u8 = 0x03;
const DISTANCE_WITH_RECEIVER_BIAS_CORRECTION: u8 = 0x04;

bitflags! {
    pub flags Status: u8 {
        const BUSY               = 0b00000001,
        const REFERENCE_OVERFLOW = 0b00000010,
        const SIGNAL_OVERFLOW    = 0b00000100,
        const INVALID_SIGNAL     = 0b00001000,
        const SECONDARY_RETURN   = 0b00010000,
        const HEALTH             = 0b00100000,
        const PROCESS_ERROR      = 0b01000000,
    }
}

pub struct LidarLiteV3<T: I2CDevice + Sized> {
    i2cdev: T
}

impl<T> LidarLiteV3<T> where T: I2CDevice + Sized
{
    pub fn new(i2cdev: T) -> Result<LidarLiteV3<T>, T::Error> {
        Ok(LidarLiteV3 { i2cdev: i2cdev })
    }

    pub fn read_device_id(&mut self) -> Result<u16, T::Error> {
        let lsb = self.i2cdev.smbus_read_byte_data(UNIT_ID_LOW)?;
        let msb = self.i2cdev.smbus_read_byte_data(UNIT_ID_HIGH)?;

        let id = LittleEndian::read_u16(&[lsb, msb]);

        Ok(id)
    }

    pub fn read_system_status(&mut self) -> Result<Status, T::Error> {
        Ok(Status::from_bits_truncate(self.i2cdev.smbus_read_byte_data(STATUS)?))
    }

    pub fn read_distance(&mut self, receiver_bias_correction: bool) -> Result<u16, T::Error> {
        self.i2cdev.smbus_write_byte_data(ACQ_COMMAND, match receiver_bias_correction {
            true => DISTANCE_WITH_RECEIVER_BIAS_CORRECTION,
            false => DISTANCE_WITHOUT_RECEIVER_BIAS_CORRECTION
        })?;

        let mut busy = true;
        while busy {
            let status = self.read_system_status()?;
            busy = status.contains(BUSY);
        }

        let lsb = self.i2cdev.smbus_read_byte_data(FULL_DELAY_LOW)?;
        let msb = self.i2cdev.smbus_read_byte_data(FULL_DELAY_HIGH)?;

        let distance = LittleEndian::read_u16(&[lsb, msb]);

        Ok(distance)
    }
}