use crate::path_finder::PackageInfo;
use image::io::Reader as ImageReader;
use image::{ImageBuffer, ImageFormat, Rgb};
use lazy_static::lazy_static;
use rusttype::{Font, Scale};
use std::cmp;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufReader, Cursor};
use walkdir::WalkDir;
use zip::{write::FileOptions, ZipWriter};

const BANNER_DIMENSIONS: (u32, u32) = (850, 400);
const ICON_DIMENSIONS: (i32, i32) = (512, 512);

lazy_static! {
  static ref FONT: Font<'static> = {
    let font_data: &[u8] = include_bytes!("../fonts/RobotoMono.ttf");
    Font::try_from_bytes(font_data).unwrap()
  };
}

pub fn package(package: &PackageInfo) {
  let mut target_dir = package.target_directory.clone();
  target_dir.push("x86_64-unknown-linux-gnu");
  target_dir.push("release");
  let mut executable = target_dir.clone();
  executable.push(&package.name);
  let mut output_file = target_dir.clone();
  output_file.push(format!("{}-devcade.zip", package.name));
  let mut file = File::create(&output_file).unwrap();
  let mut writer = ZipWriter::new(&mut file);
  let crate_root = package
    .manifest_path
    .parent()
    .expect("No parent to Cargo.toml?!");

  writer
    .add_directory("publish", FileOptions::default())
    .unwrap();

  let mut asset_dir = crate_root.to_path_buf();
  asset_dir.push("assets");

  if asset_dir.is_dir() {
    let walker = WalkDir::new(asset_dir).into_iter();

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
        log::info!("Copying asset {relative}...");
        writer.start_file(relative, FileOptions::default()).unwrap();
        let mut file = File::open(entry.path()).unwrap();
        io::copy(&mut file, &mut writer).unwrap();
      }
    }
  } else {
    log::warn!("No assets dir in crate root. Are you sure it's in the right spot?");
  }
  log::info!("Finished with assets.");

  let mut icon_path = target_dir.clone();
  icon_path.push(format!("{}-icon.png", package.name));
  let mut banner_path = target_dir.clone();
  banner_path.push(format!("{}-banner.png", package.name));

  // Write icons
  {
    log::info!("Writing store icons...");
    let mut icon_root = crate_root.to_path_buf();
    icon_root.push("store_icons");
    let mut icon = icon_root.clone();
    icon.push("icon.png");
    let mut banner = icon_root.clone();
    banner.push("banner.png");

    let mut file = File::create(&icon_path).expect("Couldn't create icon file");
    if icon.is_file() {
      let mut reader = File::open(icon).unwrap();
      let buf_reader = BufReader::new(reader.try_clone().unwrap());
      let dimensions = ImageReader::with_format(buf_reader, ImageFormat::Png)
        .into_dimensions()
        .expect("Couldn't load icon image... Is it a PNG?");
      if dimensions.0 != dimensions.1 {
        log::warn!(
          "Incorrect dimensions for icon.png: {:?}. Icons should have a square aspect ratio!",
          dimensions
        );
      }
      io::copy(&mut reader, &mut file).unwrap();
    } else {
      log::warn!(
        "Couldn't find icon (Searched: `{}`), generating one for you",
        banner.to_str().unwrap()
      );
      let (img_width, img_height) = ICON_DIMENSIONS;
      let scale = Scale::uniform(img_width as f32 / ((package.name.len() as f32 / 1.5) * 0.8));
      let (width, height) = imageproc::drawing::text_size(scale, &FONT, &package.name);

      let image = imageproc::drawing::draw_text(
        &ImageBuffer::new(img_width as u32, img_height as u32),
        Rgb([0xffu8; 3]),
        (img_width / 2) - (width / 2),
        (img_height / 2) - (height / 2),
        scale,
        &FONT,
        &package.name,
      );
      let mut png = vec![];
      image
        .write_to(&mut Cursor::new(&mut png), ImageFormat::Png)
        .unwrap();
      io::copy(&mut Cursor::new(&mut png), &mut file).unwrap();
    }
    std::mem::drop(file);

    let mut file = File::create(&banner_path).expect("Couldn't create banner file");

    if banner.is_file() {
      let mut reader = File::open(banner).unwrap();
      let buf_reader = BufReader::new(reader.try_clone().unwrap());
      let dimensions = ImageReader::with_format(buf_reader, ImageFormat::Png)
        .into_dimensions()
        .expect("Couldn't load banner image... Is it a PNG?");
      if dimensions != BANNER_DIMENSIONS {
        log::warn!(
          "Incorrect dimensions for banner.png: {:?} (Expected: {:?}). This image might display incorrectly in some applications",
          dimensions,
          BANNER_DIMENSIONS
        );
      }
      io::copy(&mut reader, &mut file).unwrap();
    } else {
      log::warn!(
        "Couldn't find banner (Searched: `{}`), generating one for you",
        banner.to_str().unwrap()
      );
      let img_width: i32 = 800;
      let img_height: i32 = 450;
      let scale = Scale::uniform(cmp::min_by(
        img_width as f32 / ((package.name.len() as f32 / 1.5) * 0.8),
        img_height as f32 * 0.8,
        |a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal),
      ));
      let (width, height) = imageproc::drawing::text_size(scale, &FONT, &package.name);
      let image = imageproc::drawing::draw_text(
        &ImageBuffer::new(img_width as u32, img_height as u32),
        Rgb([0xffu8; 3]),
        (img_width / 2) - (width / 2),
        (img_height / 2) - (height / 2),
        scale,
        &FONT,
        &package.name,
      );
      let mut png = vec![];
      image
        .write_to(&mut Cursor::new(&mut png), ImageFormat::Png)
        .unwrap();
      io::copy(&mut Cursor::new(&mut png), &mut file).unwrap();
    }
  }

  log::info!("Writing executable...");
  writer
    .start_file(format!("publish/{}", package.name), FileOptions::default())
    .unwrap();
  io::copy(&mut File::open(executable).unwrap(), &mut writer).unwrap();

  // Pretend to be monogame so @willnilges' algorithm works to find our executable
  writer
    .start_file(
      format!("publish/{}.runtimeconfig.json", package.name),
      FileOptions::default(),
    )
    .unwrap();

  log::info!("Finished packaging!");

  writer.finish().unwrap();

  log::info!("Wrote output to {}", target_dir.to_str().unwrap());
  log::info!("Upload the following files:");
  log::info!("  - {}", output_file.to_str().unwrap());
  log::info!("  - {}", icon_path.to_str().unwrap());
  log::info!("  - {}", banner_path.to_str().unwrap());
}
