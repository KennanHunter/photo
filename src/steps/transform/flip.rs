use juniper::GraphQLEnum;
use photon_rs::{transform, PhotonImage};

/// Direction Image will be flipped
#[derive(GraphQLEnum, Debug)]
pub enum FlipDirection {
    Vertical,
    Horizontal,
}

pub fn flip_image(image: &mut PhotonImage, direction: FlipDirection) -> PhotonImage {
    match direction {
        FlipDirection::Vertical => {
            transform::flipv(image);
            image.to_owned()
        }
        FlipDirection::Horizontal => {
            transform::fliph(image);
            image.to_owned()
        }
    }
}
