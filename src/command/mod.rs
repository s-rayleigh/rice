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

pub mod commands;
pub mod data;

/// `Command` for send to `Device`.
pub struct Command {
	/// Command `name`.
	pub name: &'static str,

	///Command `description`.
	pub description: &'static str,

	/// Console `command`.
	command: &'static str,

	/// Bytes of usb command beginning.
	pub usb_command_begin: &'static [u8],

	/// Bytes of usb command beginning.
	pub usb_command_end: Option<&'static [u8]>,

	/// `Type` of command data.
	pub data_type: data::Type
}

impl Command {
	pub fn find(command_name: &str) -> Option<&'static Command> {
		for command in commands::LIST {
			if command.command == command_name {
				return Some(command);
			}
		}

		None
	}
}