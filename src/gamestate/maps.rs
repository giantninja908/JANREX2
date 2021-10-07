use raylib::prelude::*;
use serde_json;

pub mod map {
    pub const BURG: &str = include_str!("../../maps/burg.json");
    pub const LITTLETOWN: &str = include_str!("../../maps/littletown.json");
    pub const SANDSTORM: &str = include_str!("../../maps/sandstorm.json");
    pub const SUBZERO: &str = include_str!("../../maps/subzero.json");
    pub const UNDERGROWTH: &str = include_str!("../../maps/undergrowth.json");
    pub const SHIPYARN: &str = include_str!("../../maps/shipyard.json");
    pub const FREIGHT: &str = include_str!("../../maps/freight.json");
    pub const LOSTWORLD: &str = include_str!("../../maps/lostworld.json");
    pub const CITADEL: &str = include_str!("../../maps/citadel.json");
    pub fn from_index(indx: u8) -> &'static str {
        if indx == 0 {
            BURG
        } else if indx == 1 {
            LITTLETOWN
        } else if indx == 2 {
            SANDSTORM
        } else if indx == 3 {
            SUBZERO
        } else if indx == 4 {
            UNDERGROWTH
        } else if indx == 5 {
            SHIPYARN
        } else if indx == 6 {
            FREIGHT
        } else if indx == 7 {
            LOSTWORLD
        } else if indx == 8 {
            CITADEL
        } else {
            panic!("Map not implemeneted yet");
        }
    }
}

#[derive(Debug)]
struct Spawn {
    pos: Vector3,
    rotation: f32,
}

#[derive(Debug)]
enum ObjectTextureVariant {
    Default,
    Classic,
    Light,
    ClassicAlt,
}

#[derive(Debug)]
enum ObjectTexture {
    Stone,
    Dirt,
    Wood,
    Grid,
    Grey,
    Default,
    Roof,
    Flag,
    Grass,
    Check,
    Lines,
    Brick,
    Link,
    Liquid,
    Grain,
    Fabric,
    Tile,
}

#[derive(Debug)]
enum ObjectType {
    Cube,
    Crate,
    Barrel,
    Ladder,
    Plane,
    CameraPos,
    Vehicle,
    Stack,
    Ramp,
    ScoreZone,
    Billboard,
    DeathZone,
    Particles,
    Objective,
    Tree,
    Cone,
    Container(Color),
    Grass,
    AcidBarrel,
    Door,
    Window,
    Flag,
    Gate,
    Checkpoint,
    WeaponPickup,
    Teleporter,
    TeddyBear,
    Trigger,
    Sign,
    DepositBox,
    LightCone,
    Camera,
    Sphere,
    Placeholder,
    CardboardBox,
    Pallet,
    Liquid,
    SoundEmitter,
    PremiumZone,
    VerifiedZone,
    CustomAsset,
    BombSite,
    BouncePad,
    TeamZone,
    Cyllinder,
    Police,
    CagedCriminal,
    ExplosiveBarrel,
    ShowcaseSkin,
    PointLight,
    Ghost,
    AiSpawner,
    Pumpkin,
    Rune,
    Skeleton,
    Knight,
}

#[derive(Debug)]
struct Object {
    position: Vector3,
    scale: Vector3,
    texture: (ObjectTexture, ObjectTextureVariant),
    visible: bool,
    collision: bool,
    wall_jumpable: bool,
    grapplable: bool,
    r#type: ObjectType,
    color: Color,
}

#[derive(Debug)]
pub struct Map {
    spawns: Vec<Spawn>,
    objects: Vec<Object>,
}

impl Map {
    /// returns a map constructed from raw text that's JSON encoded
    pub fn from_map_text<'a>(text: &'a str) -> serde_json::Result<Self> {
        let val: serde_json::Value = serde_json::from_str(text)?;
        let spawns = val["spawns"]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| {
                println!("{:?}", e);
                Spawn {
                    pos: Vector3::new(
                        e[0].as_f64().unwrap() as f32,
                        e[1].as_f64().unwrap() as f32,
                        e[2].as_f64().unwrap() as f32,
                    ),
                    rotation: e[4].as_f64().unwrap() as f32 * 90.0,
                }
            })
            .collect::<Vec<_>>();

        let cols = val["colors"]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| &e.as_str().unwrap()[1..])
            .collect::<Vec<_>>();
        let scales = {
            let a = val["xyz"].as_array().unwrap().iter().collect::<Vec<_>>();
            let mut sc: Vec<[f32; 3]> = Vec::new();
            for b in 0..a.len() / 3 {
                sc.push([
                    a[b * 3].as_f64().unwrap() as f32,
                    a[b * 3 + 1].as_f64().unwrap() as f32,
                    a[b * 3 + 2].as_f64().unwrap() as f32,
                ]);
            }
            sc
        };

        let objects = val["objects"]
            .as_array()
            .unwrap()
            .iter()
            .map(|obj| {
                if obj["i"].is_null() {
                    let pos = obj["p"].as_array().unwrap();
                    let scale = scales[obj["si"].as_u64().unwrap() as usize];
                    let is_true =
                        |val: &serde_json::Value| val.is_string() && val.as_str().unwrap() == "0";
                    Object {
                        position: Vector3::new(
                            pos[0].as_f64().unwrap() as f32,
                            pos[1].as_f64().unwrap() as f32 + (scale[1] / 2.0),
                            pos[2].as_f64().unwrap() as f32,
                        ),
                        scale: Vector3::new(scale[0], scale[1], scale[2]),
                        collision: is_true(&obj["l"]),
                        color: if obj["ci"].is_u64() {
                            Color::from_hex(cols[obj["ci"].as_u64().unwrap() as usize]).unwrap()
                        } else {
                            Color::WHITE
                        },
                        grapplable: is_true(&obj["gp"]),
                        texture: (ObjectTexture::Stone, ObjectTextureVariant::Default),
                        visible: !is_true(&obj["v"]),
                        wall_jumpable: is_true(&obj["wj"]),
                        r#type: ObjectType::Cube,
                    }
                } else {
                    Object {
                        position: Vector3::zero(),
                        scale: Vector3::one(),
                        collision: false,
                        color: Color::BLANK,
                        grapplable: false,
                        visible: false,
                        wall_jumpable: false,
                        r#type: ObjectType::AcidBarrel,
                        texture: (ObjectTexture::Stone, ObjectTextureVariant::Default),
                    }
                }
            })
            .collect::<Vec<_>>();

        Ok(Map { spawns, objects })
    }

    /// using raylib handles, render a map
    pub fn render(
        &self,
        mut rl: &mut raylib::drawing::RaylibMode3D<raylib::drawing::RaylibDrawHandle>,
        thread: &RaylibThread,
    ) {
        for obj in self.objects.iter() {
            if obj.visible {
                rl.draw_cube_v(obj.position, obj.scale, obj.color);
            }
        }
    }
}
