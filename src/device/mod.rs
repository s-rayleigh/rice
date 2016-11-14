// Copyright 2016 rayleigh
//
// This file is part of Rice.
//
// Rice is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Rice is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Rice. If not, see <http://www.gnu.org/licenses/>.

extern crate libusb;

use std::time::Duration;
use libusb::{Direction,RequestType,Recipient};
use command::Command;
use command::data;

/// `Device` structure that represents rival mouse.
/// Used for sending control commands.
pub struct Device<'a> {
	handle: libusb::DeviceHandle<'a>
}

impl<'a> Device<'a> {
	/// Constructs a new `Device` with specified vid and pid.
	///
	/// If no device with specified vid and pid, returns `Err(reason)`.
	///
	/// # Arguments
	/// * `vid` - Vendor id.
	/// * `pid` - Product id.
	pub fn new(context: &'a mut libusb::Context, vid: u16, pid: u16) -> Result<Device<'a>, String> {
		let devices = match context.devices() {
			Ok(d) => d,
			Err(_) => return Err("Cannot get devices list!".to_owned())
		};

		for device in devices.iter() {
			let descriptor = match device.device_descriptor() {
				Ok(desc) => desc,
				Err(_) => continue
			};

			if descriptor.vendor_id() == vid && descriptor.product_id() == pid {
				match device.open() {
					Ok(handle) => return Ok(Device {
						handle: handle
					}),
					Err(_) => continue
				}
			}
		}

		Err("Cannot find device with specified vid and pid.".to_owned())
	}

	pub fn send_command(&mut self, command: &Command, data: Option<String>) -> Result<(), String> {
		let mut command_bytes = Vec::new();

		command_bytes.extend_from_slice(command.usb_command_begin);

		match data {
			Some(data_string) => {
				match data::prepare(data_string, &command.data_type) {
					Some(data_bytes) => command_bytes.extend(data_bytes),
					_ => return Err("Can't prepare data for sending to device.".to_owned())
				}
			},
			_ => {}
		}

		match command.usb_command_end {
			Some(end_bytes) => command_bytes.extend_from_slice(end_bytes),
			_ => {}
		}

		let mut handle = &mut self.handle;

		let mut attach = false;

		match handle.kernel_driver_active(0) {
			Ok(true) => match handle.detach_kernel_driver(0) {
				Ok(_) => { attach = true; },
				Err(e) => return Err(format!("Can't detach kernel driver! Err: {}", e.strerror()))
			},
			_ => {}
		}

		match handle.claim_interface(0) {
			Ok(_) => {},
			Err(e) => return Err(format!("Can't claim interface! Err: {}", e.strerror()))
		}

		match handle.write_control(libusb::request_type(Direction::Out, RequestType::Class, Recipient::Interface), 0x09_u8, 0x0300_u16, 0x0000, &command_bytes, Duration::new(0, 0)) {
			Ok(_) => {},
			Err(e) => return Err(format!("Write control error: {}", e.strerror()))
		}

		match handle.release_interface(0) {
			Ok(_) => {},
			Err(e) => return Err(format!("Can't release interface! Err{}", e.strerror()))
		}

		if attach {
			match handle.attach_kernel_driver(0) {
				Ok(_) => {},
				Err(e) => return Err(format!("Can't attach kernel driver! Err: {}", e.strerror()))
			}
		}

		Ok(())
	}
}