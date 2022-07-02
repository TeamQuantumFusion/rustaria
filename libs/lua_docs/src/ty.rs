use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Type {
	Nil,
	Boolean,
	Number,
	String,
	UserData,
	Thread,
	Any,
	Void,
	This,
	Array {
		element: Box<Type>,
	},
	Table {
		key: Box<Type>,
		value: Box<Type>,
	},
	Function {
		parameters: Vec<(String, Type)>,
		ret: Option<Box<Type>>,
	},
	Union(Vec<Type>),
	Custom {
		name: String,
		generics: Vec<Type>,
	},
}

impl Display for Type {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Type::Nil => f.write_str("nil")?,
			Type::Boolean => f.write_str("boolean")?,
			Type::Number => f.write_str("number")?,
			Type::String => f.write_str("string")?,
			Type::UserData => f.write_str("userdata")?,
			Type::Thread => f.write_str("thread")?,
			Type::Any => f.write_str("any")?,
			Type::Void => f.write_str("void")?,
			Type::This => f.write_str("self")?,
			Type::Array { element } => {
				write!(f, "{element}[]")?;
			}
			Type::Table { key, value } => {
				write!(f, "table<{key}, {value}>")?;
			}
			Type::Function { parameters, ret } => {
				let parameters: Vec<String> = parameters
					.iter()
					.map(|(name, ty)| format!("{name}: {ty}"))
					.collect();
				write!(f, "fun({})", parameters.join(", "))?;
				if let Some(ret) = ret {
					write!(f, ":{ret}")?;
				}
			}
			Type::Union(types) => {
				let types: Vec<String> = types.iter().map(|ty| format!("{ty}")).collect();

				write!(f, "{}", types.join(" | "))?
			}
			Type::Custom { name, generics } => {
				let generics: Vec<String> = generics.iter().map(|ty| format!("{ty}")).collect();
				write!(f, "{name}{}", {
					if !generics.is_empty() {
						format!("<{}>", generics.join(", "))
					} else {
						"".to_string()
					}
				})?;
			}
		}

		Ok(())
	}
}
