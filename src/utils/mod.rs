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

pub fn color_to_bytes(color_str: &str) -> Option<Vec<u8>>
{
	if color_str.starts_with("#") && color_str.len() == 7 {
		let mut color_chars = color_str.chars().skip(1);

		let mut rstr = String::new();
		let mut gstr = String::new();
		let mut bstr = String::new();

		rstr.push(color_chars.next().unwrap());
		rstr.push(color_chars.next().unwrap());

		gstr.push(color_chars.next().unwrap());
		gstr.push(color_chars.next().unwrap());

		bstr.push(color_chars.next().unwrap());
		bstr.push(color_chars.next().unwrap());

		let mut color_bytes = Vec::new();

		match u8::from_str_radix(&rstr, 16) {
			Ok(b) => color_bytes.push(b),
			_ => return None
		}

		match u8::from_str_radix(&gstr, 16) {
			Ok(b) => color_bytes.push(b),
			_ => return None
		}

		match u8::from_str_radix(&bstr, 16) {
			Ok(b) => color_bytes.push(b),
			_ => return None
		}



		return Some(color_bytes);
	}

	for color in KNOWN_COLORS {
		if color_str == color.name {
			return Some(color.bytes.to_vec());
		}
	}

	None
}