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
      "--config",
      &format!(
        "package.metadata.cross.target.x86_64-unknown-linux-gnu.dockerfile.file={:?}",
        docker_path.to_str().unwrap()
      ),
      "build",
      "--release",
      "--target",
      "x86_64-unknown-linux-gnu",
      "--config",
      "term.quiet=false",
    ])
    .status()
    .ok()
    .and_then(|status| status.code())
    .and_then(|code| if code == 0 { Some(()) } else { None })
    .expect("Failed to run `cross`! Check your Dockerfile?");
}
