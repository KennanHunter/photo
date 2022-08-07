mod crop;

use std::fmt::Error;

use crate::schema::mutation::steps::Step;
use photon_rs::PhotonImage;

use self::crop::crop_image;

/// Applies the specified step to the image
pub fn step_to_func(step: Step, image: &mut PhotonImage) -> Result<PhotonImage, Error> {
    match step {
        crop => {
            let crop = crop.crop.unwrap(); // Bruh
            return Ok(crop_image(
                image,
                crop.height,
                crop.width,
                crop.xoffset,
                crop.yoffset,
            ));
        }
        _ => {
            panic!("Supply a valid step method")
        }
    }
}
