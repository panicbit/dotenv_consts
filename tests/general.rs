
mod env {
    ::dotenv_consts::dotenv_consts!();
}

#[test]
fn general() {
    assert_eq!(env::FOO, "bar");
    assert_eq!(env::PASS, "hunter2");
}
