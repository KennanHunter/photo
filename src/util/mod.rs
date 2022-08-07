use lazy_static::lazy_static;
use regex::Regex;

pub fn extract_image_base64(b64: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"^data:image/(png|jpg);base64,"#).unwrap();
    }
    RE.replace_all(b64, "").to_string()
}
