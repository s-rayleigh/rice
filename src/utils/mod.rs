
/// Known `Color`.
struct Color {
	/// Color name.
	name: &'static str,

	/// Color rbg bytes.
	bytes: &'static [u8]
}

/// List of known colors.
static KNOWN_COLORS : &'static [Color] = &[
	Color {
		name: "red",
		bytes: &[0xff_u8, 0x00_u8, 0x00_u8]
	},
	Color {
		name: "green",
		bytes: &[0x00_u8, 0xff_u8, 0x00_u8]
	},
	Color {
		name: "blue",
		bytes: &[0x00_u8, 0x00_u8, 0xff_u8]
	}
];

pub fn color_to_hex(color_str: &str) -> Option<Vec<u8>>
{
	for color in KNOWN_COLORS {
		if color_str == color.name {
			return Some(color.bytes.to_vec());
		}
	}

	None
}