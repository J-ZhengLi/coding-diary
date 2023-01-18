use crate::Result;
use anyhow::anyhow;
use clap::ArgMatches;
use std::fs::read_to_string;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use toml_edit::Document;

pub fn add(am: &ArgMatches) -> Result<()> {
    let path = am
        .try_get_one::<String>("path")?
        .map(PathBuf::from)
        .unwrap(); // path was set to required, so it's safe to unwrap
    let proj_name = path
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .ok_or_else(|| {
            anyhow!(
                "project name should only contains UTF-8 characters: {}",
                path.display()
            )
        })?;
    let is_lib = am.try_get_one::<bool>("lib")?.copied();
    let is_bin = am.try_get_one::<bool>("bin")?.copied();
    let crate_name = am.try_get_one::<String>("name")?;

    // read from workspace manifest and cache it so it could be modified later
    let ws_manifest_path = workspace_manifest_path()?;
    let mut ws_manifest_doc = manifest_doc(&ws_manifest_path)?;
    let members = ws_manifest_doc
        .as_table_mut()
        .get_mut("workspace")
        .and_then(|w| w.get_mut("members"))
        .and_then(|m| m.as_array_mut())
        .ok_or_else(|| {
            anyhow!("Incorrect workspace manifest, expecting '[workspace]' with a 'members' list")
        })?;

    let mut max_index = usize::MIN;
    for member in members.iter() {
        if let Some(member_idx) = member
            .as_str()
            .and_then(|ms| ms.split_once('_').map(|(maybe_idx, _)| maybe_idx))
            .and_then(|maybe_idx| maybe_idx.parse::<usize>().ok())
        {
            max_index = max_index.max(member_idx);
        }
    }

    let indexed_name = format!("{}_{}", max_index + 1, proj_name);
    let indexed_path = path
        .with_file_name(&indexed_name)
        .to_str()
        .ok_or_else(|| {
            anyhow!(
                "path should only contains UTF-8 characters: {}",
                path.display()
            )
        })?
        .to_owned();

    // Manually adding a indexed project name to the workspace manifest
    // this should be done first otherwise cargo will complains about not adding
    // the new project to workspace.
    println!("adding project to workspace members list...");
    members.push(indexed_path.clone());
    // Updates the workspace manifest
    std::fs::write(ws_manifest_path, ws_manifest_doc.to_string())?;

    // Run cargo to create new project
    let mut cargo_args = vec!["new", &indexed_path];
    if is_bin == Some(true) {
        cargo_args.push("--bin");
    }
    if is_lib == Some(true) {
        cargo_args.push("--lib");
    }
    if let Some(name) = crate_name {
        cargo_args.extend(["--name", name]);
    } else {
        cargo_args.extend(["--name", proj_name]);
    }

    let status = Command::new("cargo").args(cargo_args.as_slice()).status()?;

    if !status.success() {
        println!("failed to create new project, aborting...");
        return Ok(());
    }

    Ok(())
}

pub fn delete(am: &ArgMatches) -> Result<()> {
    let path = am
        .try_get_one::<String>("path")?
        .map(PathBuf::from)
        .unwrap(); // path was set to required, so it's safe to unwrap
    let path_str = path.to_str().ok_or_else(|| {
        anyhow!(
            "path contains non-UTF characters: {}",
            path.to_string_lossy()
        )
    })?;

    // Ask for confirmation
    if path.is_dir() {
        let mut stdout = io::stdout();
        writeln!(
            stdout,
            "This will remove project '{}' from workspace and all its content, \
            Are you sure? (y/N) ",
            path.display()
        )?;
        stdout.flush()?;
        let mut input: String = String::new();
        io::stdin().read_line(&mut input)?;
        writeln!(stdout)?;
        let choice = input.trim().to_lowercase();

        match choice.as_str() {
            "no" | "n" => {
                return Ok(());
            }
            s if s.is_empty() => {
                return Ok(());
            }
            _ => (),
        }
    } else {
        println!("path '{}' does not exist in the workspace", path.display());
        return Ok(());
    }

    // Then remove directory
    std::fs::remove_dir_all(&path)?;

    // Then remove the path from workspace members list
    let ws_manifest_path = workspace_manifest_path()?;
    let mut ws_manifest_doc = manifest_doc(&ws_manifest_path)?;
    let members = ws_manifest_doc
        .as_table_mut()
        .get_mut("workspace")
        .and_then(|w| w.get_mut("members"))
        .and_then(|m| m.as_array_mut())
        .ok_or_else(|| {
            anyhow!("Incorrect workspace manifest, expecting '[workspace]' with a 'members' list")
        })?;

    let mut idx_to_rm = None;
    for (idx, member) in members.iter().enumerate() {
        if let Some(m_path) = member.as_str() {
            if m_path == path_str {
                idx_to_rm = Some(idx);
                break;
            }
        }
    }
    if let Some(idx) = idx_to_rm {
        // remove this workspace member
        members.remove(idx);
        println!("'{}' was removed from workspace members", path_str);
    }
    // Updates the workspace manifest
    std::fs::write(ws_manifest_path, ws_manifest_doc.to_string())?;

    Ok(())
}

fn workspace_manifest_path() -> Result<PathBuf> {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(|p| p.join("Cargo.toml"))
        .filter(|p| p.is_file())
        .ok_or_else(|| anyhow!("This helper app should be used inside of a cargo workspace"))
}

fn manifest_doc(path: &Path) -> Result<Document> {
    let ws_manifest_content = read_to_string(path)?;
    let doc = ws_manifest_content.parse::<Document>()?;
    Ok(doc)
}
