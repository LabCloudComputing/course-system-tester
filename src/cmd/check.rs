/*
 * @Author: IceyBlackTea
 * @Date: 2022-05-02 22:07:24
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-05-03 17:30:01
 * @FilePath: /course-system-tester/src/cmd/check.rs
 * @Description: Copyright © 2022 IceyBlackTea. All rights reserved.
 */

use std::process;

fn check_tool_exist(tool: &str) -> bool {
    let cmd = format!("ls ./bin/{}", tool);
    let output = match process::Command::new("bash")
        .arg("-c")
        .arg(cmd.as_str())
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            error!("Run the check command error: {}.", err);
            return false;
        }
    };

    if output.status.success() {
        true
    } else {
        error!("{} haven't been installed.", tool);
        false
    }
}

pub fn check_tools() -> Result<(), String> {
    trace!("Checking tools...");

    let ab_go = check_tool_exist("ab-go");

    if ab_go {
        trace!("Checking tools finished.");
        return Ok(());
    }

    Err(format!(
        "Checking tools finished. Please install all tools."
    ))
}

pub fn check_privilege(password: &str) -> Result<(), String> {
    let cmd = format!("echo {} | sudo -S ls .", password);
    let output = match process::Command::new("bash")
        .arg("-c")
        .arg(cmd.as_str())
        .output()
    {
        Ok(output) => output,
        Err(err) => return Err(format!("Run the check command error: {}.", err)),
    };

    if output.status.success() {
        Ok(())
    } else {
        return Err(format!(
            "Failed to get sudo privilege. Please check your sudo password"
        ));
    }
}

pub fn check_mode(password: &str, mode: &String) -> Result<(), String> {
    check_privilege(password)?;
    check_tools()?;
    match mode.as_str() {
        "basic" => Ok(()),
        "advanced" => Err(format!(
            "Sorry, the advanced version is still not implemented."
        )),
        _ => Err(format!(
            "Unknown mode {}, which should be 'basic' or 'advanced'.",
            mode
        )),
    }
}
