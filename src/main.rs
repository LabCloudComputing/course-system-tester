/*
 * @Author: IceyBlackTea
 * @Date: 2022-05-02 18:53:09
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-05-03 18:17:06
 * @FilePath: /course-system-tester/src/main.rs
 * @Description: Copyright © 2022 IceyBlackTea. All rights reserved.
 */

mod cmd;
mod utils;

use cmd::*;
use utils::*;

use std::process;

use clap::{Parser, Subcommand};

#[macro_use]
extern crate log;
use log4rs;

/// Lab 4 Course System Tester
#[derive(Parser)]
#[clap(author, version, about = "A CLI test program for HNU Cloud Computing Lab 4, built with Rust.", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Check if tools have been installed and if get sudo privilege
    Check,
    /// Build the course system
    Build,
    /// Clean the project directory
    Clean,
    /// Test the course system including rebuilding and starting
    Run {
        #[clap(long)]
        /// Your sudo password,
        password: String,

        #[clap(long, default_value = "basic")]
        /// Test in basic or advanced mode
        mode: String,
    },
}

pub enum Version {
    Debug,
    Release,
}

fn run_cmd() -> Result<(), String> {
    match log4rs::init_file("./configs/log-config.yaml", Default::default()) {
        Ok(()) => {}
        Err(_) => {
            error!("Parse ./configs/log-config.yaml failed, use default setting.")
        }
    };

    let args = Cli::parse();

    let config_file = "./configs/tester-config.json".to_string();
    let config = config::read_config_file(config_file)?;

    match &args.command {
        Command::Check => {
            check::check_tools()?;
        }
        Command::Build => {
            let dir = config::get_json_value_as_string(&config, "directory")?;
            let build = config::get_json_value_as_string(&config, "build")?;
            build::build_system(&dir, &build)?;
        }
        Command::Clean => {
            let dir = config::get_json_value_as_string(&config, "directory")?;
            let build = config::get_json_value_as_string(&config, "clean")?;
            clean::clean(&dir, &build)?;
        }
        Command::Run { password, mode } => {
            check::check_mode(password, mode)?;
            test(Version::Release, mode, password, config)?;
        }
    }

    Ok(())
}

fn test(
    version: Version,
    mode: &String,
    password: &str,
    config: serde_json::Value,
) -> Result<(), String> {
    let dir = config::get_json_value_as_string(&config, "directory")?;

    let build = config::get_json_value_as_string(&config, "build")?;
    build::build_system(&dir, &build)?;

    let wait_seconds = config::get_json_value_as_u64(&config, "wait_seconds")?;
    let (server_cmd, base_url) = config::parse_web_server_args(&config)?;

    let mut web_server = Some(server::try_run(&dir, &server_cmd, wait_seconds)?);

    let mut store_servers = vec![];
    let (stores_cmds, mut stores_vnics) = config::parse_store_server_args(&config)?;

    warn!("Setting up all vnics...");
    for i in 0..stores_vnics.len() {
        for j in 0..stores_vnics[i].len() {
            let mut vnic = &mut stores_vnics[i][j];

            vnic::add_virtual_nic(password, format!("lo:{}{}", i, j).as_str(), &vnic.0)?;
            vnic.1 = true;
        }
    }

    for store_cmds in stores_cmds {
        for store_cmd in store_cmds {
            let store_server = Some(server::try_run(&dir, &store_cmd, wait_seconds)?);
            store_servers.push(store_server);
        }
    }

    let items = config::get_json_value(&config, "items")?;
    let mode_items = config::get_json_value(&items, "basic")?;

    let perf_items = config::get_json_value(&mode_items, "performance")?;
    let results = run::performance(&base_url, perf_items, password, &stores_vnics)?;

    server::try_kill(&mut web_server)?;

    for mut store_server in store_servers {
        server::try_kill(&mut store_server)?;
    }

    warn!("Removing up all vnics...");
    for i in 0..stores_vnics.len() {
        for j in 0..stores_vnics[i].len() {
            let mut vnic = &mut stores_vnics[i][j];

            vnic::remove_virtual_nic(password, format!("lo:{}{}", i, j).as_str())?;
            vnic.1 = false;
        }
    }

    info!("-------TESTER RESULTS------");

    let len = results.len();
    info!("Perfermance test {} times.", len);
    for i in 0..len {
        let result = results.get(i).unwrap();
        info!("No.{} choose:\n{}", i + 1, result.0);
        info!("No.{} drop:\n{}", i + 1, result.1);
    }

    info!("-------TESTER RESULTS------");

    Ok(())
}

fn main() {
    if let Err(err) = run_cmd() {
        error!("{}", err);
        error!("Tester exited with errors.");
        process::exit(-1);
    }
}
