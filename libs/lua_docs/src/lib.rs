use std::fmt::{Display, Formatter};

use crate::{ty::Type, visibility::Visibility};

pub mod ty;
pub mod visibility;

#[derive(Clone)]
pub struct Class {
	pub doc_comments: Vec<String>,

	pub name: String,
	pub parent: Option<Type>,

	pub generics: ClassGenerics,
	pub fields: Vec<Field>,
	pub functions: Vec<Function>,

	pub comment: Comment,
}

impl Display for Class {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		for line in &self.doc_comments {
			writeln!(f, "--- {}", line)?;
		}

		// @class
		write!(f, "--- @class {}{}", self.name, self.generics)?;

		if let Some(ty) = &self.parent {
			write!(f, " : {}", ty)?;
		}
		writeln!(f, "{}", self.comment)?;
		// Fields
		if !self.fields.is_empty() {
			for field in &self.fields {
				field.fmt(f)?;
			}
		}

		// Functions
		if !self.functions.is_empty() {
			// define the local for functions
			writeln!(f, "local {} = {{}};", self.name)?;
			writeln!(f)?;
			for function in &self.functions {
				function.write(self, f)?;
				writeln!(f)?;
			}
		}

		Ok(())
	}
}

#[derive(Clone, Debug)]
pub struct Field {
	pub doc_comments: Vec<String>,
	pub name: String,
	pub vis: Visibility,
	pub ty: Type,
	pub comment: Comment,
}

impl Display for Field {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		for line in &self.doc_comments {
			writeln!(f, "--- {}", line)?;
		}

		writeln!(
			f,
			"--- @field {vis}{vis_sep}{name} {ty} {comment}",
			vis = self.vis,
			vis_sep = if self.vis != Visibility::None {
				" "
			} else {
				""
			},
			name = self.name,
			ty = self.ty,
			comment = self.comment,
		)
	}
}

#[derive(Clone, Debug)]
pub struct Function {
	pub doc_comments: Vec<String>,
	pub name: String,
	pub needs_self: bool,

	pub generics: FuncGenerics,
	pub parameters: Vec<Parameter>,
	pub ret: Option<Return>,
}

impl Function {
	pub fn write(&self, parent: &Class, f: &mut Formatter<'_>) -> std::fmt::Result {
		for line in &self.doc_comments {
			writeln!(f, "--- {}", line)?;
		}

		//let mut generics = self.generics.clone();
		//for name in &parent.generics.entries {
		//    generics.entries.push((name.clone(), None));
		//}
		//self.generics.fmt(f)?;

		let mut parameters = Vec::new();
		for parameter in &self.parameters {
			parameter.fmt(f)?;
			parameters.push(parameter.name.clone());
		}

		if let Some(ret) = &self.ret {
			ret.fmt(f)?;
		}

		writeln!(
			f,
			"function {class_name}{ty}{name}({parameters})\n    -- stub\n    return nil;\nend",
			class_name = parent.name,
			ty = if self.needs_self { ':' } else { '.' },
			name = self.name,
			parameters = parameters.join(", ")
		)?;
		Ok(())
	}
}

#[derive(Clone, Debug)]
pub struct Parameter {
	pub name: String,
	pub ty: Type,
	pub comment: Comment,
}

impl Display for Parameter {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		writeln!(
			f,
			"--- @param {name} {ty} {comment}",
			name = self.name,
			ty = self.ty,
			comment = self.comment
		)
	}
}

#[derive(Clone, Debug)]
pub struct Return {
	pub ty: Type,
	pub comment: Comment,
}

impl Display for Return {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		writeln!(
			f,
			"--- @return {ty} {comment}",
			ty = self.ty,
			comment = self.comment
		)
	}
}

#[derive(Default, Clone, Debug)]
pub struct Comment {
	pub text: String,
}

impl Comment {
	pub const fn none() -> Comment {
		Comment {
			text: String::new(),
		}
	}
}

impl Display for Comment {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		if !self.text.is_empty() {
			write!(f, "@{}", self.text)
		} else {
			Ok(())
		}
	}
}

#[derive(Default, Clone, Debug)]
pub struct FuncGenerics {
	pub entries: Vec<(String, Option<Type>)>,
}

impl From<ClassGenerics> for FuncGenerics {
	fn from(generics: ClassGenerics) -> Self {
		FuncGenerics {
			entries: generics
				.entries
				.into_iter()
				.map(|name| (name, None))
				.collect(),
		}
	}
}

impl Display for FuncGenerics {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		if !self.entries.is_empty() {
			let types: Vec<String> = self
				.entries
				.iter()
				.map(|(name, ty)| {
					if let Some(ty) = ty {
						format!("{name}: {ty}")
					} else {
						name.clone()
					}
				})
				.collect();

			writeln!(f, "--- @generic {}", types.join(", "))
		} else {
			Ok(())
		}
	}
}

#[derive(Clone, Debug)]
pub struct ClassGenerics {
	pub entries: Vec<String>,
}

impl Display for ClassGenerics {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		if !self.entries.is_empty() {
			write!(f, "<{}>", self.entries.join(", "))
		} else {
			Ok(())
		}
	}
}
