
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Label {
    Single(u32),
    Multi(Vec<u32>),
    Squad((u32,u32)),
    MultiF32(Vec<f32>)
}

impl From<u32> for Label {
    fn from(x: u32) -> Self {Label::Single(x)}
}
impl From<Vec<u32>> for Label {
    fn from(x: Vec<u32>) -> Self {Label::Multi(x)}
}
impl From<(u32,u32)> for Label {
    fn from(x: (u32,u32)) -> Self {Label::Squad(x)}
}
impl From<Vec<f32>> for Label {
    fn from(x: Vec<f32>) -> Self {Label::MultiF32(x)}
}

impl Label {
    pub fn get_single(&self) -> Option<u32> {
        if let Label::Single(x) = self {
            Some(x.to_owned())
        } else {
            None
        }
    }
    pub fn get_multi(&self) -> Option<Vec<u32>> {
        if let Label::Multi(x) = self {
            Some(x.to_owned())
        } else {
            None
        }
    }
    pub fn get_multi_f32(&self) -> Option<Vec<f32>> {
        if let Label::MultiF32(x) = self {
            Some(x.to_owned())
        } else {
            None
        }
    }

    pub fn get_squad(&self) -> Option<(u32,u32)> {
        if let Label::Squad(x) = self {
            Some(x.to_owned())
        } else {
            None
        }
    }

}

