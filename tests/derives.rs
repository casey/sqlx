#[test]
#[cfg(feature = "macros")]
fn encode() {
    use sqlx::encode::Encode;

    #[derive(Encode)]
    struct Foo(i32);

    #[cfg(feature = "postgres")]
    let _: Box<dyn Encode<sqlx::Postgres>> = Box::new(Foo(1));
    #[cfg(feature = "mysql")]
    let _: Box<dyn Encode<sqlx::MySql>> = Box::new(Foo(1));
}

#[test]
#[cfg(feature = "macros")]
fn decode() {
    use sqlx::decode::Decode;

    #[derive(Decode)]
    struct Foo(i32);

    #[cfg(feature = "postgres")]
    <Foo as Decode<sqlx::Postgres>>::decode_null().ok();
    #[cfg(feature = "mysql")]
    <Foo as Decode<sqlx::MySql>>::decode_null().ok();
}
