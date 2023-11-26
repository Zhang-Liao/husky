#[salsa::debug_with_db(db = Db, jar = Jar)]
struct A(usize, usize);

#[salsa::debug_with_db(db = Db, jar = Jar)]
#[allow(dead_code)]
enum Enum {
    PropsStructVariant { a: i32 },
    TupleStructVariant(usize),
    Dog,
}

#[salsa::jar(db = Db)]
struct Jar();
