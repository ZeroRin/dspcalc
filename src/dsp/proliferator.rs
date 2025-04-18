#[derive(Debug, Clone, strum_macros::EnumIter)]
pub enum Proliferator {
    MK1,
    MK2,
    MK3,
}

const INC_LEVEL_MAX: u8 = 10;

const INC_TABLE: [f64; INC_LEVEL_MAX as usize + 1] = [
    0.0, 0.125, 0.2, 0.225, 0.25, 0.275, 0.3, 0.325, 0.35, 0.375, 0.4,
];

const ACC_TABLE: [f64; INC_LEVEL_MAX as usize + 1] =
    [0.0, 0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0, 2.25, 2.5];

const POWER_TABLE: [f64; INC_LEVEL_MAX as usize + 1] =
    [1.0, 1.3, 1.7, 2.1, 2.5, 2.9, 3.3, 3.7, 4.1, 4.5, 4.9];

impl Proliferator {
    pub const MAX_INC_LEVEL: u8 = Self::MK3.inc_level();

    #[must_use]
    pub const fn item_id(&self) -> i16 {
        match &self {
            Self::MK1 => 1141,
            Self::MK2 => 1142,
            Self::MK3 => 1143,
        }
    }

    #[must_use]
    pub const fn inc_level(&self) -> u8 {
        match &self {
            Self::MK1 => 1,
            Self::MK2 => 2,
            Self::MK3 => 4,
        }
    }

    #[must_use]
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    pub const fn life(&self, level: u8) -> u8 {
        (Self::increase(level)
            * match &self {
                Self::MK1 => 12.0,
                Self::MK2 => 24.0,
                Self::MK3 => 60.0,
            }) as u8
    }

    const fn clamp_inc_level(level: u8) -> u8 {
        if level < INC_LEVEL_MAX {
            level
        } else {
            INC_LEVEL_MAX
        }
    }

    #[must_use]
    #[allow(clippy::indexing_slicing)]
    pub const fn increase(level: u8) -> f64 {
        1.0 + INC_TABLE[Self::clamp_inc_level(level) as usize]
    }

    #[must_use]
    #[allow(clippy::indexing_slicing)]
    pub const fn accelerate(level: u8) -> f64 {
        1.0 + ACC_TABLE[Self::clamp_inc_level(level) as usize]
    }

    #[must_use]
    #[allow(clippy::indexing_slicing)]
    pub const fn power(level: u8) -> f64 {
        POWER_TABLE[Self::clamp_inc_level(level) as usize]
    }
}
