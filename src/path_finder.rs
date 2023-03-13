use crate::Args;
use cargo_metadata::{PackageId, Target};
use std::collections::HashSet;
use std::path::PathBuf;

pub struct PackageInfo {
  pub name: String,
  pub target_directory: PathBuf,
  pub manifest_path: PathBuf,
}

pub fn find_package(args: &Args) -> PackageInfo {
  let cmd = cargo_metadata::MetadataCommand::new();
  let metadata = cmd.exec().expect("Couldn't run cargo metadata...");
  let workspaces: HashSet<PackageId> = HashSet::from_iter(metadata.workspace_members.into_iter());
  let targets = metadata
    .packages
    .into_iter()
    .filter(|package| workspaces.contains(&package.id))
    .flat_map(|package| {
      let default_run = package.default_run.as_ref();
      package
        .targets
        .into_iter()
        .filter(|target| target.kind.contains(&"bin".to_owned()))
        .filter(|target: &Target| match &args.bin {
          Some(bin) => &target.name == bin,
          None => default_run.map_or(true, |name| name == &target.name),
        })
        .map(|target| (target, package.manifest_path.clone()))
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();
  if targets.len() != 1 {
    panic!("Too many possible binaries to run... Maybe you want to pass `--bin`?");
  }
  let (target, manifest_path) = targets[0].clone();
  PackageInfo {
    name: target.name,
    target_directory: metadata.target_directory.into(),
    manifest_path: manifest_path.into(),
  }
}
