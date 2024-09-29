use embedded_hal::i2c::{I2c, Error};

const ADDRESS: u8 = 0b0010_0110;

pub struct MSA301 {
	// boilerplate code goes here
}


impl MSA301 {
	pub fn get_x(&self) -> i16 {
		let mut buf = [0_u8; 2];
		self.write_read([0x02], &mut buf).unwrap();

		let mut accel = i16::from_le_bytes(buf);

		let accel = accel >> 2;

		accel
	}
}