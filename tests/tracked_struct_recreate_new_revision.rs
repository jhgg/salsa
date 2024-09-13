//! Test that a `tracked` struct on a `salsa::input` doesn't panic when recreating a new revision.
#![allow(warnings)]

use salsa::Setter;

#[salsa::input]
struct MyInput {
    field: u32,
}

#[salsa::tracked]
struct TrackedStruct<'db> {
    #[id]
    field: u32,
}

#[salsa::tracked]
fn tracked_fn(db: &dyn salsa::Database, input: MyInput) -> Option<TrackedStruct<'_>> {
    if input.field(db) == 1 {
        Some(TrackedStruct::new(db, 1))
    } else {
        None
    }
}

#[test]
fn execute() {
    let mut db = salsa::DatabaseImpl::new();
    let input = MyInput::new(&db, 1);
    assert!(tracked_fn(&db, input).is_some());
    input.set_field(&mut db).to(0);
    assert_eq!(tracked_fn(&db, input), None);
    input.set_field(&mut db).to(1);
    assert!(tracked_fn(&db, input).is_some());
}
