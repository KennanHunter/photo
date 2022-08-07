mod crop;

use crate::schema::mutation::steps::Step;
use juniper::{graphql_value, FieldError};
use photon_rs::PhotonImage;

use self::crop::crop_image;

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

    // If no step is valid then panic
    Err(FieldError::new(
        "No valid step specified",
        graphql_value!({ "internal_error": "No step specified" }),
    ))
}
