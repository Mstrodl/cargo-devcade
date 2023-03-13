use crate::path_finder::PackageInfo;
use std::fs;
use std::process::Command;

pub fn build(package: &PackageInfo) {
  let mut docker_path = package.manifest_path.parent().unwrap().to_owned();
  docker_path.push("Dockerfile.devcade");
  if !docker_path.is_file() {
    fs::write(&docker_path, include_str!("../Dockerfile.template")).unwrap();
  }
  Command::new("cross")
    .args([
      "build",
      "--release",
      "--target",
      "x86_64-unknown-linux-gnu",
      "--config",
      &format!(
        "target.x86_64-unknown-linux-musl.dockerfile.file={:?}",
        docker_path.to_str().unwrap()
      ),
    ])
    .status()
    .ok()
    .and_then(|status| status.code())
    .and_then(|code| if code == 0 { Some(()) } else { None })
    .expect("Failed to run `cross`! Check your Dockerfile?");
}
