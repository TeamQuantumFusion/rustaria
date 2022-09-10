use crate::drawer::GuiDrawer;
use crate::font::GuiFonts;

mod widget;
mod drawer;
mod font;

pub struct Gui {
	pub drawer: GuiDrawer,
	pub fonts: GuiFonts
}

impl Gui {

}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		let result = add(2, 2);
		assert_eq!(result, 4);
	}
}
