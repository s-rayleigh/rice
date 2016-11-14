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

pub fn color_to_hex(color_str: &str) -> Option<Vec<u8>>
{
	for color in KNOWN_COLORS {
		if color_str == color.name {
			return Some(color.bytes.to_vec());
		}
	}

	None
}