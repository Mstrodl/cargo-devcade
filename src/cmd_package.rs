use crate::path_finder::PackageInfo;
use std::fs::File;
use std::io;
use walkdir::WalkDir;
use zip::{write::FileOptions, ZipWriter};

pub fn package(package: &PackageInfo) {
  let mut target_dir = package.target_directory.clone();
  target_dir.push("x86_64-unknown-linux-gnu");
  target_dir.push("release");
  let mut executable = target_dir.clone();
  executable.push(&package.name);
  let mut output_file = target_dir.clone();
  output_file.push(format!("{}-devcade.zip", package.name));
  let mut file = File::create(output_file).unwrap();
  let mut writer = ZipWriter::new(&mut file);
  let crate_root = package
    .manifest_path
    .parent()
    .expect("No parent to Cargo.toml?!");
  let mut asset_dir = crate_root.to_path_buf();
  asset_dir.push("assets");
  let walker = WalkDir::new(asset_dir).into_iter();
  writer
    .add_directory("publish", FileOptions::default())
    .unwrap();
  for entry in walker {
    let entry = entry.unwrap();
    let relative = entry
      .path()
      .strip_prefix(crate_root)
      .unwrap()
      .to_str()
      .unwrap();
    let relative = format!("publish/{relative}");
    if entry.file_type().is_dir() {
      writer
        .add_directory(relative, FileOptions::default())
        .unwrap();
    } else if entry.file_type().is_file() {
      writer.start_file(relative, FileOptions::default()).unwrap();
      let mut file = File::open(entry.path()).unwrap();
      io::copy(&mut file, &mut writer).unwrap();
    }
  }

  // Write icons
  {
    let mut icon_root = crate_root.to_path_buf();
    icon_root.push("store_icons");
    let mut icon = icon_root.clone();
    icon.push("icon.png");
    writer
      .start_file("icon.png", FileOptions::default())
      .unwrap();
    io::copy(&mut File::open(icon).unwrap(), &mut writer).unwrap();
    let mut banner = icon_root.clone();
    banner.push("banner.png");
    writer
      .start_file("banner.png", FileOptions::default())
      .unwrap();
    io::copy(&mut File::open(banner).unwrap(), &mut writer).unwrap();
  }

  writer.finish().unwrap();
}
