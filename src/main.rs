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

mod device;
mod command;
mod utils;

use std::env;
use command::Command;
use device::Device;

const VID: u16 = 0x1038;
const PID: u16 = 0x1700;

fn main() {
	let mut context = match libusb::Context::new() {
		Ok(c) => c,
		Err(e) => {
			println!("Can't create libusb context!\nReason: {}", e);
			return;
		}
	};

	let mut args = env::args();

	//Skip program path
	args.next();

	let command_name = match args.next() {
		Some(com) => com,
		None => {
			println!("You must specify command.");
			return;
		}
	};

	let command = match Command::find(command_name.as_str()) {
		Some(com) => com,
		None => {
			println!("Cannot find command.");
			return;
		}
	};

	let command_data = args.next();

	let mut device = match Device::new(&mut context, VID, PID) {
		Ok(d) => d,
		Err(e) => {
			println!("Can't construct device!\nReason: {}", e);
			return;
		}
	};

	match device.send_command(&command, command_data) {
		Ok(_) => println!("Ok"),
		Err(err) => println!("{}", err)
	}
}