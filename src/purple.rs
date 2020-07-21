#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::all
)]

use glib_sys::{GHashTable, GList};

include!(concat!(env!("OUT_DIR"), "/purple.rs"));
