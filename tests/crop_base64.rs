#[test]
fn crop_base64() {
    let encoded_value: String = base64::encode(std::fs::read("tests/static/amiyama.jpg").unwrap());

    let query: &str = &format!(
        "{}{}{}",
        r#"mutation {createExchange(input: {auth: "epic jwt here", imageBase64: ""#,
        encoded_value,
        r#"", steps: {crop: {width: 100, height: 100, xoffset: 0, yoffset: 0}}}) {uploadLink}}"#
    );

    futures::executor::block_on(juniper::execute(
        query,
        None,
        &photo::schema::create_schema(),
        &juniper::Variables::new(),
        &(),
    ))
    .unwrap();
}
