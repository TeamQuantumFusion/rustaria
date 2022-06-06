use bitflags::bitflags;
bitflags! {
	pub struct Modifiers: u8 {
		const Shift       = 0x0001;
		const Control     = 0x0002;
		const Alt         = 0x0004;
		const Super       = 0x0008;
		const CapsLock    = 0x0010;
		const NumLock     = 0x0020;
	}
}