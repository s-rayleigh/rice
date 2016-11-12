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