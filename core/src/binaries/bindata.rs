use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::Path,
};

pub fn write_file(name: &str, from: &str, to: &str) {
    create_dir_all(&Path::new(to).parent().unwrap()).unwrap();
    let mut file = File::create(to).unwrap();

    write_header(&mut file, name, from);
    // write_asset(&mut file, from);
}

fn write_header(file: &mut File, name: &str, from: &str) {
    let header = format!(
        "// Code generated by Prisma Client Rust. DO NOT EDIT.

use prisma_client_rust::binaries;

pub fn init() {{
    binaries::unpack(DATA, \"{}\");
}}

const DATA: &[u8] = include_bytes!(\"{}\");
",
        name, from
    );

    file.write(header.as_bytes()).unwrap();
}

fn write_asset(file: &mut File, from: &str) {
    let mut engine_file = File::open(from).unwrap();

    uncompressed_memcopy(file, &mut engine_file);
}

fn uncompressed_memcopy(file: &mut File, from: &mut File) {
    file.write(b"const DATA: &[u8] = ").unwrap();

    let mut buffer = Vec::new();

    from.read_to_end(&mut buffer).unwrap();

    file.write(format!("{:?}", buffer).as_bytes()).unwrap();
}
