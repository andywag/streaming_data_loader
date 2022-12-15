
use serde::{Deserialize, Serialize};
use pyo3::prelude::*;


#[derive(Clone, Debug, Serialize, Deserialize, FromPyObject)]
pub enum Label {
    Single(u32),
    Multi(Vec<u32>),
    Squad((u32,u32)),
    MultiF32(Vec<f32>),
    Masking(Vec<i32>),
    MaskPositionLabel((Vec<u32>, Vec<i32>))
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
impl From<Vec<i32>> for Label {
    fn from(x: Vec<i32>) -> Self {Label::Masking(x)}
}
impl From<(Vec<u32>,Vec<i32>)> for Label {
    fn from(x: (Vec<u32>, Vec<i32>)) -> Self {Label::MaskPositionLabel(x)}
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
    pub fn get_vec_i32(&self) -> Option<Vec<i32>> {
        if let Label::Masking(x) = self {
            Some(x.to_owned())
        } else {
            None
        }
    }
    pub fn get_vec_u32_i32(&self) -> Option<(Vec<u32>, Vec<i32>)> {
        if let Label::MaskPositionLabel(x) = self {
            Some(x.to_owned())
        } else {
            None
        }
    }
    pub fn get_masked_position(&self) -> Option<Vec<u32>> {
        if let Label::MaskPositionLabel(x) = self {
            Some(x.0.to_owned())
        } else {
            None
        }
    }
    pub fn get_masked_label(&self) -> Option<Vec<i32>> {
        if let Label::MaskPositionLabel(x) = self {
            Some(x.1.to_owned())
        } else {
            None
        }
    }


}

