use std::path::Path;

use egui::{
    ecolor::HexColor as Color,
    ColorImage,
};
use serde::{Deserialize, Serialize};

use crate::error::applicationerror::ApplicationError;

use super::{load_image_from_path, pixel_index};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct ThemeData {
    dark: Color,         // 0,0
    dark_accent: Color,  //1,0
    light: Color,        //0,3
    light_accent: Color, //0,2
    grey: Color,         //0,1

    error: Color,        //2,1
    warning: Color,      //2,4
    accept: Color,       //5,2
    link: Color,         //1,4
    link_visited: Color, //1,2

    spore: Color,   //3,4
    blossom: Color, //4,2
    bonus: Color,   //4,1
}

impl ThemeData {
    pub fn new(data: [&str; 13]) -> Result<Self, ApplicationError> {
        Ok(Self {
            dark: data[0].parse()?,
            dark_accent: data[1].parse()?,
            light: data[2].parse()?,
            light_accent: data[3].parse()?,
            grey: data[4].parse()?,
            error: data[5].parse()?,
            warning: data[6].parse()?,
            accept: data[7].parse()?,
            link: data[8].parse()?,
            link_visited: data[9].parse()?,
            spore: data[10].parse()?,
            blossom: data[11].parse()?,
            bonus: data[12].parse()?,
        })
    }
    pub fn from_image_file(file: &Path) -> Result<Self, ApplicationError> {
        let image = load_image_from_path(file)?;
        image.try_into()
    }
}

impl Default for ThemeData {
    fn default() -> Self {
        Self {
            dark: "#2d2922".parse().unwrap(),
            dark_accent: "#15181f".parse().unwrap(),
            light: "#fefefe".parse().unwrap(),
            light_accent: "#c2c2c2".parse().unwrap(),
            grey: "#737373".parse().unwrap(),
            error: "#81261c".parse().unwrap(),
            warning: "#ece184".parse().unwrap(),
            accept: "#5c8239".parse().unwrap(),
            link: "#82c7e4".parse().unwrap(),
            link_visited: "#3e7687".parse().unwrap(),
            spore: "#febc47".parse().unwrap(),
            blossom: "#ea90c9".parse().unwrap(),
            bonus: "#d8396a".parse().unwrap(),
        }
    }
}

impl TryFrom<ColorImage> for ThemeData {
    type Error = ApplicationError;

    fn try_from(value: ColorImage) -> Result<Self, Self::Error> {
        let pixels = value
            .pixels
            .into_iter()
            .map(Color::Hex4)
            .collect::<Vec<_>>();
        // should be 35 different colors
        if pixels.len() < 35 {
            return Err(ApplicationError::ImageSize);
        }
        Ok(Self {
            dark: pixels[pixel_index(0, 0)],
            dark_accent: pixels[pixel_index(1, 0)],
            light: pixels[pixel_index(0, 3)],
            light_accent: pixels[pixel_index(0, 2)],
            grey: pixels[pixel_index(0, 1)],
            error: pixels[pixel_index(2, 1)],
            warning: pixels[pixel_index(2, 4)],
            accept: pixels[pixel_index(5, 2)],
            link: pixels[pixel_index(1, 4)],
            link_visited: pixels[pixel_index(1, 2)],
            spore: pixels[pixel_index(3, 4)],
            blossom: pixels[pixel_index(4, 2)],
            bonus: pixels[pixel_index(4, 1)],
        })
    }
}
