use std::cmp::min;

use juniper::GraphQLEnum;
use photon_rs::{conv, PhotonImage};

/// Direction Image will be flipped
#[derive(GraphQLEnum, Debug)]
pub enum Blurs {
    Gaussian,
    Box,
}

pub fn blur_image(image: &mut PhotonImage, blur: Blurs, radius: Option<i32>) -> PhotonImage {
    match blur {
        Blurs::Gaussian => {
            conv::box_blur(image);
            image.to_owned()
        }
        Blurs::Box => {
            conv::gaussian_blur(
                image,
                radius.unwrap_or(((min(image.get_height(), image.get_width())) / 2) as i32),
            );
            image.to_owned()
        }
    }
}
