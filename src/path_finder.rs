use crate::Args;
use cargo_metadata::{PackageId, Target};
use std::collections::HashSet;
use std::path::PathBuf;

pub struct PackageInfo {
  pub name: String,
  pub target_directory: PathBuf,
  pub manifest_path: PathBuf,
  pub has_devcade_feature: bool,
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
      let has_devcade_feature = package.features.contains_key("devcade");
      package
        .targets
        .into_iter()
        .filter(|target| target.kind.contains(&"bin".to_owned()))
        .filter(|target: &Target| match &args.bin {
          Some(bin) => &target.name == bin,
          None => default_run.map_or(true, |name| name == &target.name),
        })
        .map(|target| (target, package.manifest_path.clone(), has_devcade_feature))
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();
  if targets.len() != 1 {
    log::error!("Too many possible binaries to run... Maybe you want to pass `--bin`?");
    todo!();
  }
  let (target, manifest_path, has_devcade_feature) = targets[0].clone();
  PackageInfo {
    name: target.name,
    target_directory: metadata.target_directory.into(),
    manifest_path: manifest_path.into(),
    has_devcade_feature,
  }
}
