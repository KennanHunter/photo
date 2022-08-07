#[test]
fn baseline_query() {
    futures::executor::block_on(juniper::execute(
        "query { ok }",
        None,
        &photo::schema::create_schema(),
        &juniper::Variables::new(),
        &(),
    ))
    .unwrap()
    .0
    .as_object_value()
    .unwrap();
}
