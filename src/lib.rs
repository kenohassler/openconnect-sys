#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(rustdoc::broken_intra_doc_links)]

use libc::{addrinfo, time_t, uid_t};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
