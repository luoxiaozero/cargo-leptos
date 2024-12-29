use crate::config::{BuildConfig, BuildModeConfig};
use clap::{Args, Subcommand};
use color_eyre::Result;
use serde::{Deserialize, Serialize};
use xshell::{cmd, Shell};

#[derive(Debug, Clone, Deserialize, Serialize, Subcommand, PartialEq)]
pub enum BuildCommand {
    Csr,
    Ssr(BuildSsrArgs),
    Hydrate,
}

#[derive(Clone, Debug, Args, PartialEq, Eq, Serialize, Deserialize)]
#[clap(about)]
pub struct BuildSsrArgs {
    #[clap(long, short, action)]
    pub no_hydrate: bool,
}

impl BuildCommand {
    pub fn run(&self, config: &BuildConfig) -> Result<()> {
        let Some(output_name) = &config.output_name else {
            unreachable!();
        };
        match self {
            Self::Csr => build(output_name, &config.csr),
            BuildCommand::Ssr(BuildSsrArgs { no_hydrate }) => {
                if !no_hydrate {
                    build(output_name, &config.hydrate)?;
                }
                build(output_name, &config.ssr)?;
                Result::Ok(())
            }
            BuildCommand::Hydrate => build(output_name, &config.hydrate),
        }
    }
}

pub fn build(name: &String, config: &BuildModeConfig) -> Result<()> {
    let cargo_args = if config.args.is_empty() {
        let mut cargo_args = vec![
            "build".to_string(),
            format!("--package={}", name),
            "--no-default-features".to_string(),
            format!("--target={}", config.target),
        ];

        if config.lib {
            cargo_args.push(format!("--lib={}", name));
        } else {
            cargo_args.push(format!("--bin={}", name));
        }

        cargo_args
    } else {
        config.args.clone()
    };

    let sh = Shell::new()?;
    Ok(cmd!(sh, "cargo {cargo_args...}").run()?)
}
