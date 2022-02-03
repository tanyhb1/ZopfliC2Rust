#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(extern_types)]
#![feature(linkage)]
#![feature(main)]
#![feature(ptr_wrapping_offset_from)]
#![feature(register_tool)]
#![register_tool(c2rust)]

extern crate libc;

pub mod src {
    pub mod blocksplitter;
    pub mod cache;
    pub mod deflate;
    pub mod gzip_container;
    pub mod hash;
    pub mod katajainen;
    pub mod lz77;
    pub mod squeeze;
    pub mod tree;
    pub mod util;
    pub mod zlib_container;
    pub mod zopfli_lib;
} // mod src
