#[derive(Debug)]
pub enum Biome {
    Mountain,
    Plains,
    Forest,
    Desert,
    ShallowWater,
    DeepWater,
    Snow,
}

impl Biome {
    pub fn cost(&self) -> Option<u32> {
        match self {
            Biome::Mountain => None,
            Biome::Plains => Some(1),
            Biome::Forest => Some(2),
            Biome::Desert => Some(3),
            Biome::ShallowWater => Some(4),
            Biome::DeepWater => None,
            Biome::Snow => Some(6),
        }
    }
}
