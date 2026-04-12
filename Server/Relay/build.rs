use std::env;
use std::path::PathBuf;

fn main() {
	let proto_file = "../Protocol/schematics/thorium.proto";
	let includes = "../Protocol/schematics/";
	prost_build::Config::new()
		.compile_protos(&[proto_file], &[includes])
		.expect("Compilation error");

	println!("cargo:return-if-changed={}", proto_file);
}
