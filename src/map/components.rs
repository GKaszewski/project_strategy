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

    pub fn from_elevation_and_moisture(elevation: f64, moisture: f64) -> Biome {
        if elevation < 0.0 {
            if moisture < 0.1 {
                Biome::DeepWater
            } else if moisture < 0.2 {
                Biome::ShallowWater
            } else {
                Biome::Plains
            }
        } else if elevation < 0.1 {
            if moisture < 0.33 {
                Biome::ShallowWater
            } else if moisture < 0.66 {
                Biome::Plains
            } else {
                Biome::Forest
            }
        } else if elevation < 0.2 {
            if moisture < 0.16 {
                Biome::ShallowWater
            } else if moisture < 0.33 {
                Biome::Plains
            } else if moisture < 0.66 {
                Biome::Forest
            } else {
                Biome::Mountain
            }
        } else if elevation < 0.3 {
            if moisture < 0.16 {
                Biome::Plains
            } else if moisture < 0.33 {
                Biome::Forest
            } else {
                Biome::Mountain
            }
        } else if elevation < 0.4 {
            if moisture < 0.16 {
                Biome::Forest
            } else {
                Biome::Mountain
            }
        } else {
            Biome::Mountain
        }
    }
}
