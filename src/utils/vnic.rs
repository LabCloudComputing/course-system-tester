/*
 * @Author: IceyBlackTea
 * @Date: 2022-05-02 19:44:46
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-05-03 16:06:02
 * @FilePath: /course-system-tester/src/utils/vnic.rs
 * @Description: Copyright © 2022 IceyBlackTea. All rights reserved.
 */

use std::process;

pub fn add_virtual_nic(password: &str, vnic_name: &str, vnic_ip: &str) -> Result<(), String> {
    let cmd = format!(
        "echo {} | sudo -S ifconfig {} {}",
        password, vnic_name, vnic_ip
    );

    let output = match process::Command::new("bash")
        .arg("-c")
        .arg(cmd.as_str())
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            return Err(format!(
                "Adding virtual nic {} {} error: {}.",
                vnic_name, vnic_ip, err
            ));
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "Adding virtual nic {} {} error: {}.",
            vnic_name, vnic_ip, stderr
        ));
    }

    Ok(())
}

pub fn remove_virtual_nic(password: &str, vnic_name: &str) -> Result<(), String> {
    let cmd = format!("echo {} | sudo -S ifconfig {} down", password, vnic_name);

    let output = match process::Command::new("bash")
        .arg("-c")
        .arg(cmd.as_str())
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            return Err(format!(
                "Removing virtual nic {} error: {}.",
                vnic_name, err
            ));
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        warn!(
            "{}",
            format!("Removing virtual nic {} error: {}.", vnic_name, stderr)
        );
    }

    Ok(())
}
