use raylib::prelude::*;
use serde_json;

mod map {
    pub const BURG: &str = include_str!("../../maps/burg.json");
    pub const LITTLETOWN: &str = include_str!("../../maps/littletown.json");
}

struct Spawn {
    pos: Vector3,
    rotation: f32,
}

enum ObjectTextureVariant {
    Default,
    Classic,
    Light,
    ClassicAlt,
}

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

struct Map {
    spawns: Vec<Spawn>,
    objects: Vec<Object>,
}

impl Map {
    pub fn from_map_text<'a>(text: &'a str) -> serde_json::Result<Self> {
        let val: serde_json::Value = serde_json::from_str(text)?;
        let spawns = val["spawns"]
            .as_array()
            .iter()
            .map(|e| Spawn {
                pos: Vector3::new(
                    e[0].as_f64().unwrap() as f32,
                    e[1].as_f64().unwrap() as f32,
                    e[2].as_f64().unwrap() as f32,
                ),
                rotation: e[4].as_f64().unwrap() as f32 * 90.0,
            })
            .collect::<Vec<_>>();

        let cols = val["colors"]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| &e.as_str().unwrap()[1..])
            .collect::<Vec<_>>();
        let scales = {
            let a = val["xyz"].as_array().unwrap().iter();
            0
        };

        let objects = val["objects"]
            .as_array()
            .unwrap()
            .iter()
            .map(|obj| {
                let pos = obj["p"].as_array().unwrap();
                let scale = Vector3::zero();
                let is_true =
                    |val: &serde_json::Value| val.is_string() && val.as_str().unwrap() == "0";
                Object {
                    position: Vector3::new(
                        pos[0].as_f64().unwrap() as f32,
                        pos[1].as_f64().unwrap() as f32,
                        pos[2].as_f64().unwrap() as f32,
                    ),
                    scale,
                    collision: is_true(&obj["l"]),
                    color: Color::from_hex(cols[obj["ci"].as_u64().unwrap() as usize]).unwrap(),
                    grapplable: is_true(&obj["gp"]),
                    texture: (ObjectTexture::Stone, ObjectTextureVariant::Default),
                    visible: is_true(&obj["v"]),
                    wall_jumpable: is_true(&obj["wj"]),
                    r#type: ObjectType::Cube,
                }
            })
            .collect::<Vec<_>>();

        Ok(Map {
            spawns,
            objects: Vec::new(),
        })
    }
    pub fn render(
        &self,
        mut rl: &mut raylib::drawing::RaylibMode3D<raylib::drawing::RaylibDrawHandle>,
        thread: &RaylibThread,
    ) {
        for obj in self.objects.iter() {
            rl.draw_cube_v(obj.position, obj.scale, obj.color);
        }
    }
}
