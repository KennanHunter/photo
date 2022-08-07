use crate::schema::mutation::steps::Step;
use photon_rs::PhotonImage;

pub fn step_to_func(step: Step, image: &mut PhotonImage) {
    println!("{:#?}", step)
}
