use photon_rs::PhotonImage;

pub fn crop_image(image: &mut PhotonImage, height: i32, width: i32, xoffset: i32, yoffset: i32) {
    photon_rs::transform::crop(
        image,
        xoffset.try_into().unwrap(),
        yoffset.try_into().unwrap(),
        (xoffset + width).try_into().unwrap(),
        (yoffset + height).try_into().unwrap(),
    );
}
