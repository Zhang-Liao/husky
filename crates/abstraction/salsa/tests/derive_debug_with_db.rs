#[salsa::derive_debug_with_db(db = Db, jar = Jar)]
struct A(usize, usize);

#[salsa::derive_debug_with_db(db = Db, jar = Jar)]
enum Enum {
    RegularStructVariant { a: i32 },
    TupleStructVariant(usize),
    Dog,
}

trait Db: salsa::DbWithJar<Jar> {}

#[salsa::jar(db = Db)]
struct Jar();