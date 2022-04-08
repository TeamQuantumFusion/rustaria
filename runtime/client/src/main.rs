use std::str::FromStr;
use std::thread::sleep;
use std::time::{Duration, Instant};
use rustaria::api::Api;
use rustaria::api::prototype::tile::TilePrototype;
use rustaria::world::chunk::{Chunk, ChunkLayer};
use rustaria::network::Networking;
use rustaria::network::packet::{ClientPacket, ServerPacket};
use rustaria::Server;
use rustaria::world::tile::Tile;
use rustaria::world::World;
use rustaria_api::prototype::Prototype;
use rustaria_api::tag::Tag;
use rustaria_graphics::{RenderBackend, RenderHandler};
use rustaria_network::networking::{ClientNetworking, ServerNetworking};
use rustaria_util::{debug, info};
use rustaria_util::ty::ChunkPos;
use rustaria_wgpu::WgpuBackend;

mod network;

fn main() {
    rustaria_util::initialize().unwrap();

    let mut api = Api::new();
    api.reload().unwrap();

    let mut server = Server {
        network: Networking {
            internal: ServerNetworking::new(None).unwrap(),
        },
        world: World::new(),
    };
    let networking = ClientNetworking::join_local(&mut server.network.internal);


    server.world.put_chunk(ChunkPos { x: 0, y: 0 }, Chunk {
        tiles: ChunkLayer::new(api.get_registry::<TilePrototype>().create_from_tag(&Tag::from_str("rustaria:air").unwrap()).unwrap())
    });

    let mut client = Client {
        api,
        world: Some(ClientWorld {
            networking,
            integrated: Some(Box::new(server)),
        }),
    };

    let mut render = RenderHandler::<WgpuBackend>::new();
    while render.alive() {
        render.poll(|event| {});
        client.tick();
        render.draw();
    }
}

pub struct Client {
    pub api: Api,
    pub world: Option<ClientWorld>,
}

pub struct ClientWorld {
    pub networking: ClientNetworking<ServerPacket, ClientPacket>,
    pub integrated: Option<Box<Server>>,
}

impl Client {
    pub fn tick(&mut self) {
        if let Some(world) = &mut self.world {
            if let Some(integrated) = &mut world.integrated {
                integrated.tick();
            }

            world.networking.poll(|packet| {
            })
        }
        sleep(Duration::from_millis(100));
    }
}
