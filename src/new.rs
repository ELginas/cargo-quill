use anyhow::bail;
use argh::FromArgs;
use convert_case::{Case, Casing};
use path_clean::PathClean;
use std::env;
use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

const CARGO_TOML_FILE_TEXT: &str = r#"
[lib]
crate-type = ["cdylib"]

[dependencies]
quill = { git = "https://github.com/feather-rs/feather", branch = "main" }"#;

const LIB_RS_FILE_TEXT: &str = r#"use quill::{Game, Plugin, Setup};

quill::plugin!($plugin_name);

struct $plugin_name {}

impl Plugin for $plugin_name {
    fn enable(_game: &mut Game, _setup: &mut Setup<Self>) -> Self {
        $plugin_name {}
    }

    fn disable(self, _game: &mut Game) {}
}
"#;

#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "new")]
/// Create Quill plugin project.
pub struct New {
    #[argh(positional)]
    path: PathBuf,
}

struct Config<'a> {
    name: &'a OsStr,
    path: PathBuf,
}

pub fn new(args: New) -> anyhow::Result<()> {
    if args.path.exists() {
        bail!("Path already exists.");
    }

    let config = Config {
        name: args.path.file_name().unwrap(),
        path: absolute_path(&args.path)?,
    };

    let mut command = cargo_new_command(&config);
    let status = command.spawn()?.wait()?;
    if !status.success() {
        bail!("build failed");
    }

    change_cargo_toml(&config)?;
    change_lib_rs(&config)?;

    println!("Plugin project {:?} created.", config.name);
    Ok(())
}

fn cargo_new_command(config: &Config) -> Command {
    let mut cmd = Command::new("cargo");
    cmd.arg("new");
    cmd.arg("--lib");
    cmd.arg(config.name.clone());

    cmd.stdout(Stdio::piped());

    cmd
}

fn change_cargo_toml(config: &Config) -> anyhow::Result<()> {
    let mut path = config.path.clone();
    path.push("Cargo.toml");

    let mut lines = read_lines(&path)?;
    lines.pop();
    lines.pop();

    let mut file = OpenOptions::new().write(true).open(path)?;
    for line in lines {
        file.write(line.as_bytes())?;
        file.write("\n".as_bytes())?;
    }
    file.write(CARGO_TOML_FILE_TEXT.as_bytes())?;

    Ok(())
}

fn change_lib_rs(config: &Config) -> anyhow::Result<()> {
    let mut path = config.path.clone();
    path.push("src");
    path.push("lib.rs");

    let mut file = OpenOptions::new().write(true).open(path)?;

    let name_str = String::from(config.name.clone().to_str().unwrap());
    let plugin_name = name_str.to_case(Case::Pascal) + "Plugin";
    let text = LIB_RS_FILE_TEXT.replace("$plugin_name", &plugin_name[..]);
    file.write(text.as_bytes())?;

    Ok(())
}

fn read_lines(path: &PathBuf) -> anyhow::Result<Vec<String>> {
    let mut file = File::open(path)?;
    let reader = BufReader::new(&mut file);

    let lines: Vec<_> = reader.lines().map(|l| l.unwrap()).collect();

    Ok(lines)
}

fn absolute_path(path: &PathBuf) -> anyhow::Result<PathBuf> {
    let absolute_path = if path.is_absolute() {
        path.clone()
    } else {
        env::current_dir()?.join(path)
    }
    .clean();

    Ok(absolute_path)
}
