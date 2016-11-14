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

use utils;

/// The `Type` of command data.
pub enum Type {
	/// Command has no data.
	None,
	Color
}

pub fn prepare(data: String, data_type: &Type) -> Option<Vec<u8>> {
	match *data_type {
		Type::None => return None,
		Type::Color => {
			return utils::color_to_bytes(&data);
		}
	}
}