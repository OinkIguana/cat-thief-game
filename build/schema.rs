#[derive(Deserialize)]
pub struct SpriteSpec {
    pub image: String,
    pub frames: Vec<[u32; 4]>,
}

#[derive(Deserialize)]
pub struct FontSpec {
    pub styles: Vec<Font>,
}

#[derive(Deserialize)]
pub struct Font {
    pub name: String,
    pub file: String,
    pub sizes: Vec<u32>,
}

#[derive(Copy, Clone, Deserialize)]
pub struct Dimen {
    pub width: u32,
    pub height: u32,
}

impl ::std::fmt::Display for Dimen {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Dimen::new({}, {})", self.width, self.height)
    }
}

#[derive(Copy, Clone, Deserialize)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl ::std::fmt::Display for Point {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Point::new({}, {})", self.x, self.y)
    }
}

#[derive(Deserialize)]
pub struct TileSetSpec {
    pub image: String,
    pub count: u32,
    pub per_row: u32,
    pub size: Dimen,
    pub origin: Point,
    pub spacing: Dimen,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Tile {
    None,
    Some(usize, u32),
}

#[derive(Deserialize)]
pub struct TileGridSpec {
    pub tile_sets: Vec<String>,
    pub offset: Option<Point>,
    pub size: Dimen,
    pub tiles: Vec<Tile>,
}

#[derive(Deserialize)]
pub struct TiledTMXSpec {
    pub width: String,
    pub height: String,
    pub tilewidth: String,
    pub tileheight: String,
    #[serde(rename = "tileset", default)]
    pub tilesets: Vec<TiledTMXTileset>,
    #[serde(rename = "layer", default)]
    pub layers: Vec<TiledTMXLayer>,
}

pub struct TiledSpec {
    pub width: u32,
    pub height: u32,
    pub tilewidth: u32,
    pub tileheight: u32,
    pub tilesets: Vec<TiledTileset>,
    pub layers: Vec<TiledLayer>,
}

impl TiledTMXSpec {
    pub fn resolve(self) -> TiledSpec {
        TiledSpec {
            width: self.width.parse::<u32>().unwrap(),
            height: self.height.parse::<u32>().unwrap(),
            tilewidth: self.tilewidth.parse::<u32>().unwrap(),
            tileheight: self.tileheight.parse::<u32>().unwrap(),
            tilesets: self.tilesets.into_iter().map(TiledTMXTileset::resolve).collect(),
            layers: self.layers.into_iter().map(TiledTMXLayer::resolve).collect(),
        }
    }
}

#[derive(Deserialize)]
pub struct TiledTMXTileset {
    pub name: String,
    pub firstgid: String,
    pub tilecount: String,
}

impl TiledTMXTileset {
    fn resolve(self) -> TiledTileset {
        TiledTileset {
            name: self.name,
            firstgid: self.firstgid.parse::<u32>().unwrap(),
            tilecount: self.tilecount.parse::<u32>().unwrap(),
        }
    }
}

pub struct TiledTileset {
    pub name: String,
    pub firstgid: u32,
    pub tilecount: u32,
}

#[derive(Deserialize)]
pub struct TiledTMXLayer {
    pub name: String,
    pub data: TiledTMXData,
}

impl TiledTMXLayer {
    fn resolve(self) -> TiledLayer {
        TiledLayer {
            name: self.name,
            tiles: self.data.value
                .split(",")
                .map(|index| index.trim().parse::<u32>().unwrap())
                .collect()
        }
    }
}

pub struct TiledLayer {
    pub name: String,
    pub tiles: Vec<u32>,
}

#[derive(Deserialize)]
pub struct TiledTMXData {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Deserialize)]
pub struct DialogSpec {
    pub rules: Vec<Rule>,
    pub messages: Vec<Message>,
}

#[derive(Deserialize)]
pub struct Rule {
    pub name: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Clone, Deserialize)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

#[derive(Deserialize)]
pub struct Message {
    pub speaker: Option<String>,
    pub message: String,
}
