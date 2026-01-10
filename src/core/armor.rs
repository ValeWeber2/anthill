#[derive(Clone,Copy)]
pub enum ArmorQuality {
    Poor,
    Common,
    Good,
    Superior,
    Legendary,
}

impl ArmorQuality {
    pub fn mitigation_multiplier(&self) -> u32 {
        match self {
            ArmorQuality::Poor => 1,
            ArmorQuality::Common => 2,
            ArmorQuality::Good => 3,
            ArmorQuality::Superior => 5,
            ArmorQuality::Legendary => 8,
        }
    }
}

pub struct Armor {
    pub base_mitigation: u32,
    pub quality: ArmorQuality,
}

impl Armor {
    pub fn mitigation(&self) -> u32 {
        self.base_mitigation * self.quality.mitigation_multiplier()
    }
}