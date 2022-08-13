use photon_rs::{transform::SamplingFilter, PhotonImage};

pub fn resize_image(image: &mut PhotonImage, height: i32, width: i32) -> PhotonImage {
    let resized_img = photon_rs::transform::resize(
        image,
        height.try_into().unwrap(),
        width.try_into().unwrap(),
        SamplingFilter::Nearest,
    );

    resized_img
}
