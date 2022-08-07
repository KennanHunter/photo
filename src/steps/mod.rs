mod crop;

use crate::schema::mutation::steps::Step;
use photon_rs::PhotonImage;

use self::crop::crop_image;

pub fn step_to_func(step: Step, image: &mut PhotonImage) {
    match step {
        crop => {
            let crop = crop.crop.unwrap(); // Bruh
            crop_image(image, crop.height, crop.width, crop.xoffset, crop.yoffset)
        }
        _ => {
            panic!("Supply a valid step method")
        }
    }
}
