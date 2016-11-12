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
			return utils::color_to_hex(&data);
		}
	}
}