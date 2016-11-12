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