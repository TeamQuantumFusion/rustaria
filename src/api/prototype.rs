#[macro_export]
macro_rules! pt {
    ( $($TY:ty),* => $B:block) => {
        $({
        type P = $TY;
        $B
    };)*
    };
}

#[macro_export]
macro_rules! prototypes {
    ($B:block) => {
        $crate::pt!($crate::chunk::layer::tile::TilePrototype, $crate::entity::prototype::EntityPrototype => $B);
    };
}
