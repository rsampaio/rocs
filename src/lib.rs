#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate prettytable;
extern crate dirs;
extern crate openapi;
extern crate reqwest;
extern crate rocl;
extern crate rusqlite;
extern crate serde;
extern crate serde_json;
extern crate spinners;
extern crate time;
extern crate uuid;
extern crate valico;

pub mod cli;
pub mod ext;
pub mod models;
pub mod store;
