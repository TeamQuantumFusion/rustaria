use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Visibility {
	None,
	Public,
	Protected,
	Private,
}

impl Display for Visibility {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Visibility::None => Ok(()),
			Visibility::Public => f.write_str("public"),
			Visibility::Protected => f.write_str("protected"),
			Visibility::Private => f.write_str("private"),
		}
	}
}
