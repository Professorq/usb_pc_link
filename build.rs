use std::fs;
use std::env;
use std::error::Error;
use mustache::MapBuilder;
extern crate mustache;

fn main() -> Result<(), Box<dyn Error>> {
    let unit_file_name = "usb_pc_link.service";

    let source = fs::read_to_string(format!("{unit_file_name}.template"))?;
    let template = mustache::compile_str(&source).unwrap();

    let mut unit_file = fs::File::options().write(true).truncate(true).create(true)
	.open(unit_file_name)?;

    let data = MapBuilder::new()
	.insert_str("project_root", env::current_dir()?.to_str().unwrap())
	.insert_str("profile", env::var("PROFILE").unwrap())
	.build();
    template.render_data(&mut unit_file, &data).unwrap();
    Ok(())
}
