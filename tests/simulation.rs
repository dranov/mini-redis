
#[tokio::test]
async fn assert_madsim() {
    assert!(cfg!(madsim));
}