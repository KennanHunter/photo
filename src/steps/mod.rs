pub mod transform;

use crate::schema::mutation::steps::Step;
use juniper::{graphql_value, FieldError};
use photon_rs::PhotonImage;

use self::transform::{crop::crop_image, flip::flip_image, resize::resize_image};

/// Applies the specified step to the image
pub fn step_to_func(step: Step, image: &mut PhotonImage) -> Result<PhotonImage, FieldError> {
    if step.crop.is_some() {
        let crop = step.crop.unwrap(); // Bruh
        return Ok(crop_image(
            image,
            crop.height,
            crop.width,
            crop.xoffset,
            crop.yoffset,
        ));
    }
    if step.resize.is_some() {
        let resize = step.resize.unwrap(); // Bruh
        return Ok(resize_image(image, resize.height, resize.width));
    }
    if step.flip.is_some() {
        let flip = step.flip.unwrap();
        return Ok(flip_image(image, flip.flip_direction));
    }

    // If no step is valid then panic
    Err(FieldError::new(
        "No valid step specified",
        graphql_value!({ "internal_error": "No step specified" }),
    ))
}
