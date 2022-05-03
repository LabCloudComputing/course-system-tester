/*
 * @Author: IceyBlackTea
 * @Date: 2022-05-02 20:09:52
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-05-02 20:57:44
 * @FilePath: /course-tester/src/utils/store.rs
 * @Description: Copyright © 2022 IceyBlackTea. All rights reserved.
 */

use std::process;
use std::thread;
use std::time::Duration;

fn run(dir: &String, bin: &String, server_args: &String) -> Result<process::Child, String> {
    let cmd = format!("{} {}", bin, server_args);

    match process::Command::new("bash")
        .current_dir(dir)
        .arg("-c")
        .arg(cmd.as_str())
        .spawn()
    {
        Ok(child) => Ok(child),
        Err(err) => Err(format!("Can't run your store server: {}.", err)),
    }
}

pub fn try_run(
    dir: &String,
    bin: &String,
    server_args: &String,
    wait_seconds: u64,
) -> Result<process::Child, String> {
    let mut server = run(dir, bin, server_args)?;
    trace!(
        "Waiting in {}s for the store server to start...",
        wait_seconds
    );
    thread::sleep(Duration::from_secs(wait_seconds));
    match server.try_wait() {
        Ok(Some(_)) => Err(format!("The store server isn't running.")),
        Ok(None) => Ok(server),
        Err(err) => Err(format!("Can't wait the store server to run: {}.", err)),
    }
}

pub fn try_kill(server: &mut Option<process::Child>) -> Result<(), String> {
    match server {
        Some(server) => {
            warn!("Trying to kill the store server...");
            match server.kill() {
                Ok(()) => {
                    trace!("Waiting in 1s for server to stop...");
                    thread::sleep(Duration::from_secs(1));
                    trace!("The store server is stopped.");
                    Ok(())
                }
                Err(err) => Err(format!("Kill the store server failed: {}.", err)),
            }
        }
        None => {
            warn!("The store server didn't run.");
            Ok(())
        }
    }
}
