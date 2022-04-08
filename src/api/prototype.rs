pub mod tile;

macro_rules! pt {
    ( $($TY:ty),* => $B:block) => {
        $({
        type P = $TY;
        $B
    };)*
    };
}

pub fn test() {
}

#[macro_export]
macro_rules! prototypes {
    ($B:block) => {
        pt!(crate::api::prototype::tile::TilePrototype => $B);
    };
}
