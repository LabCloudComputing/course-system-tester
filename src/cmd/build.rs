/*
 * @Author: IceyBlackTea
 * @Date: 2022-05-02 22:06:32
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-05-03 17:21:40
 * @FilePath: /course-system-tester/src/cmd/build.rs
 * @Description: Copyright © 2022 IceyBlackTea. All rights reserved.
 */

use std::process;

pub fn build_system(dir: &String, cmd: &String) -> Result<(), String> {
    trace!("Building system...");
    let output = match process::Command::new("bash")
        .current_dir(dir.as_str())
        .arg("-c")
        .arg(cmd.as_str())
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            return Err(format!("Run system build command error: {}.", err));
        }
    };

    if output.status.success() {
        trace!("Building system finished.");
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Building system error:\n{}", stderr))
    }
}
