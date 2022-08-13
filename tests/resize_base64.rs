use juniper::ScalarValue;
use photo::util::extract_image_base64;
use photon_rs::{base64_to_image, native::open_image};

#[test]
fn resize_base64() {
    let encoded_value: String = open_image("static/kurisu.png")
        .expect("File should open")
        .get_base64();

    let query: &str = &format!(
        "{}{}{}",
        r#"mutation {createExchange(input: {auth: "epic jwt here", imageBase64: ""#,
        encoded_value,
        r#"", steps: {resize: {width: 100, height: 100}}}) {base64image}}"#
    );

    let res = futures::executor::block_on(juniper::execute(
        query,
        None,
        &photo::schema::create_schema(),
        &juniper::Variables::new(),
        &(),
    ))
    .unwrap();

    let b64str = res
        .0
        .as_object_value()
        .unwrap()
        .get_field_value("createExchange")
        .unwrap()
        .as_object_value()
        .unwrap()
        .get_field_value("base64image")
        .unwrap()
        .as_scalar()
        .unwrap()
        .as_str()
        .unwrap();

    // println!("{:#?}", b64str.bytes());

    let img = base64_to_image(&extract_image_base64(&b64str));
    println!(
        "Height: {:?}, Width: {:?}",
        img.get_height(),
        img.get_width()
    );

    assert!(img.get_height() == 100);

    #[cfg(debug_assertions)]
    {
        std::fs::create_dir_all("tests/tmp").unwrap();
        photon_rs::native::save_image(img, "tests/tmp/resized_image.jpg");
    }
}
