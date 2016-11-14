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

use command::Command;
use command::data::Type;

/// List of `commands`.
pub static LIST : &'static [Command] = &[
	Command {
		name: "Wheel color",
		description: "Changes wheel color to specified.",
		command: "wheel-color",
		usb_command_begin: &[0x05_u8, 0x00_u8, 0x01_u8],
		usb_command_end: Some(&[0xff_u8, 0x32_u8, 0xc8_u8, 0xc8_u8, 0x00_u8, 0x01_u8, 0x01_u8]),
		data_type: Type::Color
	},
	Command {
		name: "Logo color",
		description: "Changes logo color to specified.",
		command: "logo-color",
		usb_command_begin: &[0x05_u8, 0x00_u8, 0x00_u8],
		usb_command_end: Some(&[0xff_u8, 0x32_u8, 0xc8_u8, 0xc8_u8, 0x00_u8, 0x00_u8, 0x01_u8]),
		data_type: Type::Color
	}
];