#![deny(warnings)]

#[macro_use]
extern crate cfg_if;
#[macro_use]
extern crate diesel;

mod helpers;
mod schema;

mod as_changeset;
mod identifiable;
mod insertable;
