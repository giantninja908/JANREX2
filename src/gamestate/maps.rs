use raylib::prelude::*;
use serde_json;
use std::collections::HashMap;

pub mod map {
    // retarded to load a maps into memory but its faster than loading on request
    pub const BURG:        &str = include_str!("../../maps/jsons/00_Burg.json");
    pub const LITTLETOWN:  &str = include_str!("../../maps/jsons/01_Littletown.json");
    pub const SANDSTORM:   &str = include_str!("../../maps/jsons/02_Sandstorm.json");
    pub const SUBZERO:     &str = include_str!("../../maps/jsons/03_Subzero.json");
    pub const UNDERGROWTH: &str = include_str!("../../maps/jsons/04_Undergrowth.json");
    pub const SHIPMENT:    &str = include_str!("../../maps/jsons/05_Shipment.json"); // old name "05_Shipyard.json"
    pub const FREIGHT:     &str = include_str!("../../maps/jsons/06_Freight.json");
    pub const LOSTWORLD:   &str = include_str!("../../maps/jsons/07_Lostworld.json");
    pub const CITADEL:     &str = include_str!("../../maps/jsons/08_Citadel.json");
    pub const OASIS:       &str = include_str!("../../maps/jsons/09_Oasis.json");
    pub const KANJI:       &str = include_str!("../../maps/jsons/10_Kanji.json");
    pub const INDUSTRY:    &str = include_str!("../../maps/jsons/11_Industry.json");
    pub const LUMBER:      &str = include_str!("../../maps/jsons/12_Lumber.json");
    pub const EVACUATION:  &str = include_str!("../../maps/jsons/13_Evacuation.json");
    pub const SITE:        &str = include_str!("../../maps/jsons/14_Site.json");
    pub const SKYTEMPLE:   &str = include_str!("../../maps/jsons/15_SkyTemple.json");
    pub const LAGOON:      &str = include_str!("../../maps/jsons/16_Lagoon.json");
    pub const BUREAU:      &str = include_str!("../../maps/jsons/17_Bureau.json");
    pub const TORTUGA:     &str = include_str!("../../maps/jsons/18_Tortuga.json");
    pub const TROPICANO:   &str = include_str!("../../maps/jsons/19_Tropicano.json");

    // map idx is just how they get sent from the source code
    pub fn from_index(indx: u8) -> &'static str {
        match indx {
            0 => BURG,
            1 => LITTLETOWN,
            2 => SANDSTORM,
            3 => SUBZERO,
            4 => UNDERGROWTH,
            5 => SHIPMENT,
            6 => FREIGHT,
            7 => LOSTWORLD,
            8 => CITADEL,
            9 => OASIS,
            10 => KANJI,
            11 => INDUSTRY,
            12 => LUMBER,
            13 => EVACUATION,
            14 => SITE,
            15 => SKYTEMPLE,
            16 => LAGOON,
            17 => BUREAU,
            18 => TORTUGA,
            19 => TROPICANO,
            _ => panic!("Map not implemeneted yet: {}", indx)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Spawn {
    pub pos: Vector3,
    pub rotation: f32,
}

#[derive(Debug)]
enum ObjectTextureVariant {
    Default,
    Classic,
    Light,
    ClassicAlt,
}

#[derive(Debug, PartialEq, Eq, Hash)]
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
pub struct Object {
    pub position: Vector3,
    pub scale: Vector3,
    pos_max: Vector3,
    pos_min: Vector3,
    texture: (ObjectTexture, ObjectTextureVariant),
    visible: bool,
    pub collision: bool,
    wall_jumpable: bool,
    grapplable: bool,
    r#type: ObjectType,
    color: Color,
    emission: Option<Color>,
}

#[derive(Debug)]
pub struct Map {
    pub spawns: Vec<Spawn>,
    pub objects: Vec<Object>,
    textures: HashMap<ObjectTexture, Texture2D>,
}

impl Map {
    /// returns a map constructed from raw text that's JSON encoded
    pub fn from_map_text<'a>(
        text: &'a str,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
    ) -> serde_json::Result<Self> {
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
                    let tval = if obj["t"].is_u64() {
                        obj["t"].as_u64().unwrap()
                    } else {
                        0
                    };

                    let texval = if tval == 0 {
                        ObjectTexture::Stone
                    } else if tval == 1 {
                        ObjectTexture::Dirt
                    } else if tval == 2 {
                        ObjectTexture::Wood
                    } else if tval == 3 {
                        ObjectTexture::Grid
                    } else if tval == 4 {
                        ObjectTexture::Grey
                    } else {
                        ObjectTexture::Default
                    };
                    Object {
                        position: Vector3::new(
                            pos[0].as_f64().unwrap() as f32,
                            pos[1].as_f64().unwrap() as f32 + (scale[1] / 2.0),
                            pos[2].as_f64().unwrap() as f32,
                        ),
                        scale: Vector3::new(scale[0], scale[1], scale[2]),
                        pos_max: Vector3::new(
                            pos[0].as_f64().unwrap() as f32 + scale[0],
                            pos[1].as_f64().unwrap() as f32 + scale[1],
                            pos[2].as_f64().unwrap() as f32 + scale[2],
                        ),
                        pos_min: Vector3::new(
                            pos[0].as_f64().unwrap() as f32 - scale[0],
                            pos[1].as_f64().unwrap() as f32 - scale[1],
                            pos[2].as_f64().unwrap() as f32 - scale[2],
                        ),
                        collision: !is_true(&obj["l"]),
                        color: if obj["ci"].is_u64() {
                            Color::from_hex(cols[obj["ci"].as_u64().unwrap() as usize]).unwrap()
                        } else {
                            Color::WHITE
                        },
                        grapplable: is_true(&obj["gp"]),
                        texture: (texval, ObjectTextureVariant::Default),
                        visible: !is_true(&obj["v"]),
                        wall_jumpable: is_true(&obj["wj"]),
                        r#type: ObjectType::Cube,
                        emission: if obj["ci"].is_u64() {
                            Some(
                                Color::from_hex(cols[obj["ci"].as_u64().unwrap() as usize])
                                    .unwrap(),
                            )
                        } else {
                            None
                        },
                    }
                } else {
                    Object {
                        position: Vector3::zero(),
                        scale: Vector3::one(),
                        pos_max: Vector3::zero(),
                        pos_min: Vector3::zero(),
                        collision: false,
                        color: Color::BLANK,
                        grapplable: false,
                        visible: false,
                        wall_jumpable: false,
                        r#type: ObjectType::AcidBarrel,
                        texture: (ObjectTexture::Stone, ObjectTextureVariant::Default),
                        emission: None,
                    }
                }
            })
            .collect::<Vec<_>>();

        let mut textures = HashMap::new();
        macro_rules! insert_tex_to_textu {
            ($rl: expr, $thread: expr, $textures: expr, $txt: expr, $typ: expr) => {
                $textures.insert(
                    $typ,
                    $rl.load_texture_from_image(
                        $thread,
                        &Image::load_image_from_mem(
                            "png",
                            &$txt.iter().map(|&e| e).collect::<Vec<_>>(),
                            $txt.len() as i32,
                        )
                        .unwrap(),
                    )
                    .unwrap(),
                );
            };
        }

        insert_tex_to_textu!(
            rl,
            thread,
            textures,
            include_bytes!("../../maps/textures/brick_0.png"),
            ObjectTexture::Brick
        );
        insert_tex_to_textu!(
            rl,
            thread,
            textures,
            include_bytes!("../../maps/textures/wall_0.png"),
            ObjectTexture::Stone
        );
        insert_tex_to_textu!(
            rl,
            thread,
            textures,
            include_bytes!("../../maps/textures/dirt_0.png"),
            ObjectTexture::Dirt
        );
        insert_tex_to_textu!(
            rl,
            thread,
            textures,
            include_bytes!("../../maps/textures/floor_0.png"),
            ObjectTexture::Wood
        );
        insert_tex_to_textu!(
            rl,
            thread,
            textures,
            include_bytes!("../../maps/textures/grid_0.png"),
            ObjectTexture::Grid
        );
        insert_tex_to_textu!(
            rl,
            thread,
            textures,
            include_bytes!("../../maps/textures/grey_0.png"),
            ObjectTexture::Grey
        );

        Ok(Map {
            spawns,
            objects,
            textures,
        })
    }

    /// using raylib handles, render a map
    pub fn render(
        &mut self,
        mut rl: &mut raylib::drawing::RaylibMode3D<raylib::drawing::RaylibDrawHandle>,
        thread: &RaylibThread,
    ) {
        for obj in self.objects.iter() {
            if obj.visible {
                match &obj.texture.0 {
                    ObjectTexture::Default => {
                        rl.draw_cube_v(
                            obj.position,
                            obj.scale,
                            match &obj.emission {
                                Some(e) => e,
                                None => &obj.color,
                            },
                        );
                    }
                    _ => match &obj.emission {
                        Some(e) => rl.draw_cube_v(obj.position, obj.scale, e),
                        None => Map::render_cube(
                            self.textures.get(&ObjectTexture::Brick).unwrap(),
                            obj.position,
                            obj.scale,
                            obj.color,
                        ),
                    },
                }
                // rl.draw_cube_v(obj.position, obj.scale, obj.color);
            }
        }
    }

    fn render_cube(texture: &Texture2D, pos: Vector3, scale: Vector3, color: Color) {
        let (x, y, z) = (pos.x, pos.y, pos.z);
        let (width, height, length) = (scale.x, scale.y, scale.z);
        use raylib::ffi::*;
        unsafe {
            rlEnableTexture(texture.id);
            rlBegin(0x0007);
            rlColor4ub(color.r, color.g, color.b, color.a);
            // Front Face
            rlNormal3f(0.0, 0.0, 1.0); // Normal Pointing Towards Viewer
            rlTexCoord2f(0.0, 0.0);
            rlVertex3f(x - width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Left Of The Texture and Quad
            rlTexCoord2f(width / 100.0, 0.0);
            rlVertex3f(x + width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Right Of The Texture and Quad
            rlTexCoord2f(width / 100.0, height / 100.0);
            rlVertex3f(x + width / 2.0, y + height / 2.0, z + length / 2.0); // Top Right Of The Texture and Quad
            rlTexCoord2f(0.0, height / 100.0);
            rlVertex3f(x - width / 2.0, y + height / 2.0, z + length / 2.0); // Top Left Of The Texture and Quad
                                                                             // Back Face
            rlNormal3f(0.0, 0.0, -1.0); // Normal Pointing Away From Viewer
            rlTexCoord2f(width / 100.0, 0.0);
            rlVertex3f(x - width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Right Of The Texture and Quad
            rlTexCoord2f(width / 100.0, height / 100.0);
            rlVertex3f(x - width / 2.0, y + height / 2.0, z - length / 2.0); // Top Right Of The Texture and Quad
            rlTexCoord2f(0.0, height / 100.0);
            rlVertex3f(x + width / 2.0, y + height / 2.0, z - length / 2.0); // Top Left Of The Texture and Quad
            rlTexCoord2f(0.0, 0.0);
            rlVertex3f(x + width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Left Of The Texture and Quad
                                                                             // Top Face
            rlNormal3f(0.0, 1.0, 0.0); // Normal Pointing Up
            rlTexCoord2f(0.0, length / 100.0);
            rlVertex3f(x - width / 2.0, y + height / 2.0, z - length / 2.0); // Top Left Of The Texture and Quad
            rlTexCoord2f(0.0, 0.0);
            rlVertex3f(x - width / 2.0, y + height / 2.0, z + length / 2.0); // Bottom Left Of The Texture and Quad
            rlTexCoord2f(width / 100.0, 0.0);
            rlVertex3f(x + width / 2.0, y + height / 2.0, z + length / 2.0); // Bottom Right Of The Texture and Quad
            rlTexCoord2f(width / 100.0, length / 100.0);
            rlVertex3f(x + width / 2.0, y + height / 2.0, z - length / 2.0); // Top Right Of The Texture and Quad
                                                                             // Bottom Face
            rlNormal3f(0.0, -1.0, 0.0); // Normal Pointing Down
            rlTexCoord2f(width / 100.0, length / 100.0);
            rlVertex3f(x - width / 2.0, y - height / 2.0, z - length / 2.0); // Top Right Of The Texture and Quad
            rlTexCoord2f(0.0, length / 100.0);
            rlVertex3f(x + width / 2.0, y - height / 2.0, z - length / 2.0); // Top Left Of The Texture and Quad
            rlTexCoord2f(0.0, 0.0);
            rlVertex3f(x + width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Left Of The Texture and Quad
            rlTexCoord2f(width / 100.0, 0.0);
            rlVertex3f(x - width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Right Of The Texture and Quad
                                                                             // Right face
            rlNormal3f(1.0, 0.0, 0.0); // Normal Pointing Right
            rlTexCoord2f(length / 100.0, 0.0);
            rlVertex3f(x + width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Right Of The Texture and Quad
            rlTexCoord2f(length / 100.0, height / 100.0);
            rlVertex3f(x + width / 2.0, y + height / 2.0, z - length / 2.0); // Top Right Of The Texture and Quad
            rlTexCoord2f(0.0, height / 100.0);
            rlVertex3f(x + width / 2.0, y + height / 2.0, z + length / 2.0); // Top Left Of The Texture and Quad
            rlTexCoord2f(0.0, 0.0);
            rlVertex3f(x + width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Left Of The Texture and Quad
                                                                             // Left Face
            rlNormal3f(-1.0, 0.0, 0.0); // Normal Pointing Left
            rlTexCoord2f(0.0, 0.0);
            rlVertex3f(x - width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Left Of The Texture and Quad
            rlTexCoord2f(length / 100.0, 0.0);
            rlVertex3f(x - width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Right Of The Texture and Quad
            rlTexCoord2f(length / 100.0, height / 100.0);
            rlVertex3f(x - width / 2.0, y + height / 2.0, z + length / 2.0); // Top Right Of The Texture and Quad
            rlTexCoord2f(0.0, height / 100.0);
            rlVertex3f(x - width / 2.0, y + height / 2.0, z - length / 2.0); // Top Left Of The Texture and Quad
            rlEnd();
            rlDisableTexture();
        }
    }
}


// Collisions quick test code (needs to be completely rewritten)
pub struct PlayerBox {
    pub position: Vector3,
    pub scale: Vector3,
    pub pos_max: Vector3,
    pub pos_min: Vector3,
}

#[allow(non_snake_case)]
pub fn is_point_inside_AABB(point: Vector3, obj: &Object) -> bool {
    (point.x >= obj.pos_min.x && point.x <= obj.pos_max.x) &&
    (point.y >= obj.pos_min.y && point.y <= obj.pos_max.y) &&
    (point.z >= obj.pos_min.z && point.z <= obj.pos_max.z)
}

pub fn intersect(p: &PlayerBox, obj: &Object) -> bool {
    (p.pos_min.x <= obj.pos_max.x && p.pos_max.x >= obj.pos_min.x) &&
    (p.pos_min.y <= obj.pos_max.y && p.pos_max.y >= obj.pos_min.y) &&
    (p.pos_min.z <= obj.pos_max.z && p.pos_max.z >= obj.pos_min.z)
}