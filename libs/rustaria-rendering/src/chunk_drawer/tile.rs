use rustaria::{api::{prototype::tile::TilePrototype, ty::ConnectionType}, world::tile::Tile};
use rustaria_api::{registry::Registry, ty::RawId};
use rustariac_backend::{
    builder::VertexBuilder,
    ty::{AtlasLocation, PosTexture, Rectangle}, ClientBackend,};

// todo dynamic this.
const VARIATIONS: u8 = 3;

pub struct TileDrawer {
    image: AtlasLocation,
}

impl TileDrawer {
    pub fn new(
        prototype: &TilePrototype,
        backend: &ClientBackend,
    ) -> Option<TileDrawer> {
        let instance = backend.instance();
        let tag = prototype.sprite.as_ref()?;
        let location = instance.atlas.get(&tag);
        Some(TileDrawer { image: location })
    }

    pub fn push(
        &self,
        builder: &mut VertexBuilder<PosTexture>,
        x: f32,
        y: f32,
        kind: TileConnectionKind,
    ) {
        let (kind_x, kind_y) = kind.get_tex_pos();
        let tile_height = self.image.height / 4.0;
        let tile_width = self.image.width / (4.0 * VARIATIONS as f32);

        builder.quad((
            Rectangle {
                x,
                y,
                width: 1.0,
                height: 1.0,
            },
            AtlasLocation {
                x: self.image.x + (kind_x * tile_width),
                y: (self.image.y + (kind_y * tile_height)) + tile_height,
                width: tile_width,
                height: -tile_height,
            },
        ));
    }
}

#[derive(Clone, Copy)]
pub struct BakedTile {
    pub id: RawId,
    pub connection: ConnectionType,
}

impl BakedTile {
    pub fn new(
        registry: &Registry<TilePrototype>,
        tile: &Tile,
    ) -> Option<BakedTile> {
        if let Some(TilePrototype {
            sprite: Some(_),
            connection,
            ..
        }) = registry.get_prototype(tile.id)
        {
            Some(BakedTile {
                id: tile.id,
                connection: *connection
            })
        } else {
            None
        }

    }

}


#[derive(Copy, Clone)]
pub enum TileConnectionKind {
    Solid,
    Vertical,
    Horizontal,
    Standalone,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    UpFlat,
    DownFlat,
    LeftFlat,
    RightFlat,
    UpEnd,
    DownEnd,
    LeftEnd,
    RightEnd,
}

impl TileConnectionKind {
    pub fn new(
        up: ConnectionType,
        down: ConnectionType,
        left: ConnectionType,
        right: ConnectionType,
    ) -> TileConnectionKind {
        use ConnectionType::{Connected, Isolated};
        match (up, down, left, right) {
            (Connected, Connected, Connected, Connected) => TileConnectionKind::Solid,
            (Isolated, Isolated, Isolated, Isolated) => TileConnectionKind::Standalone,
            (Connected, Connected, Isolated, Isolated) => TileConnectionKind::Vertical,
            (Isolated, Isolated, Connected, Connected) => TileConnectionKind::Horizontal,

            (Isolated, Connected, Connected, Connected) => TileConnectionKind::UpFlat,
            (Isolated, Connected, Isolated, Isolated) => TileConnectionKind::UpEnd,
            (Isolated, Connected, Isolated, Connected) => TileConnectionKind::UpLeft,
            (Isolated, Connected, Connected, Isolated) => TileConnectionKind::UpRight,

            (Connected, Isolated, Connected, Connected) => TileConnectionKind::DownFlat,
            (Connected, Isolated, Isolated, Isolated) => TileConnectionKind::DownEnd,
            (Connected, Isolated, Isolated, Connected) => TileConnectionKind::DownLeft,
            (Connected, Isolated, Connected, Isolated) => TileConnectionKind::DownRight,

            (Connected, Connected, Isolated, Connected) => TileConnectionKind::LeftFlat,
            (Isolated, Isolated, Isolated, Connected) => TileConnectionKind::LeftEnd,
            (Connected, Connected, Connected, Isolated) => TileConnectionKind::RightFlat,
            (Isolated, Isolated, Connected, Isolated) => TileConnectionKind::RightEnd,
            _ => TileConnectionKind::Solid,
        }
    }

    pub fn get_tex_pos(self) -> (f32, f32) {
        match self {
            TileConnectionKind::Solid => (0.0, 3.0),
            TileConnectionKind::Vertical => (0.0, 2.0),
            TileConnectionKind::Horizontal => (1.0, 3.0),
            TileConnectionKind::Standalone => (1.0, 2.0),
            TileConnectionKind::UpLeft => (0.0, 0.0),
            TileConnectionKind::UpRight => (1.0, 0.0),
            TileConnectionKind::DownLeft => (0.0, 1.0),
            TileConnectionKind::DownRight => (1.0, 1.0),
            TileConnectionKind::UpFlat => (3.0, 0.0),
            TileConnectionKind::DownFlat => (3.0, 1.0),
            TileConnectionKind::LeftFlat => (3.0, 2.0),
            TileConnectionKind::RightFlat => (3.0, 3.0),
            TileConnectionKind::UpEnd => (2.0, 0.0),
            TileConnectionKind::DownEnd => (2.0, 1.0),
            TileConnectionKind::LeftEnd => (2.0, 2.0),
            TileConnectionKind::RightEnd => (2.0, 3.0),
        }
    }
}
