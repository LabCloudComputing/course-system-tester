/*
 * @Author: IceyBlackTea
 * @Date: 2022-05-03 14:09:41
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-05-03 17:17:15
 * @FilePath: /course-system-tester/src/cmd/run/perf.rs
 * @Description: Copyright © 2022 IceyBlackTea. All rights reserved.
 */

use std::process;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

use crate::utils::{config, vnic};
use crate::Version;

pub fn performance(
    base_url: &String,
    items: serde_json::Value,
    password: &str,
    vnics: &Vec<std::vec::Vec<(std::string::String, bool)>>,
) -> Result<Vec<(String, String)>, String> {
    trace!("Testing performance...");

    match items.as_array() {
        Some(perf) => {
            let results = match run_ab_go(base_url, perf, password, vnics) {
                Ok(results) => results,
                Err(err) => {
                    return Err(err);
                }
            };

            trace!("Testing performance finished.");
            Ok(results)
        }
        None => {
            return Err(format!(
                "Performance item is '{:?}', which should be an array.",
                items
            ));
        }
    }
}

pub fn run_ab_go(
    base_url: &String,
    perf: &Vec<serde_json::Value>,
    password: &str,
    vnics: &Vec<std::vec::Vec<(std::string::String, bool)>>,
) -> Result<Vec<(String, String)>, String> {
    let mut results = vec![];

    for item in perf {
        let choose_item = config::get_json_value(item, "choose")?;
        let choose_child = run_single_ab_test(base_url, &choose_item)?;

        let drop_item = config::get_json_value(item, "drop")?;
        let drop_child = run_single_ab_test(base_url, &drop_item)?;

        let password_str = password.to_string();
        let vnic_name_a = vnics[0][3].0.to_string();
        let vnic_name_b = vnics[1][3].0.to_string();

        let (tx, rx) = mpsc::channel();
        thread::spawn(move || loop {
            vnic::remove_virtual_nic(password_str.as_str(), "lo:03").unwrap();
            vnic::remove_virtual_nic(password_str.as_str(), "lo:13").unwrap();

            thread::sleep(Duration::from_millis(200));

            vnic::add_virtual_nic(password_str.as_str(), "lo:03", vnic_name_a.as_str()).unwrap();
            vnic::add_virtual_nic(password_str.as_str(), "lo:13", vnic_name_b.as_str()).unwrap();

            thread::sleep(Duration::from_millis(200));

            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }
        });

        let choose_output = choose_child.wait_with_output().unwrap();
        let drop_output = drop_child.wait_with_output().unwrap();

        let mut choose_result = String::new();
        let mut drop_result = String::new();

        if choose_output.status.success() {
            let output = String::from_utf8_lossy(&choose_output.stdout)
                .as_ref()
                .to_string();
            trace!("ab-go choose stdout output:\n{}", output);
            choose_result = output;
        } else {
            let output = String::from_utf8_lossy(&choose_output.stderr)
                .as_ref()
                .to_string();
            error!("ab-go choose test failed:\n {}.", output);
        }

        if drop_output.status.success() {
            let output = String::from_utf8_lossy(&drop_output.stdout)
                .as_ref()
                .to_string();
            trace!("ab-go drop stdout output:\n{}", output);
            drop_result = output;
        } else {
            let output = String::from_utf8_lossy(&drop_output.stderr)
                .as_ref()
                .to_string();
            error!("ab-go drop test failed:\n {}.", output);
        }

        let _ = tx.send(());
        results.push((choose_result, drop_result))
    }

    Ok(results)
}

fn run_single_ab_test(
    base_url: &String,
    item: &serde_json::Value,
) -> Result<process::Child, String> {
    let path = config::get_json_value_as_string(item, "path")?;
    let requests = config::get_json_value_as_u64(item, "requests")?;
    let concurrency = config::get_json_value_as_u64(item, "concurrency")?;
    let postdata = config::get_json_value_as_string(item, "postdata")?;
    let port = config::get_json_value_as_u64(item, "port")?;

    let cmd = format!(
        "./bin/ab-go -H \"content-type: application/json\" -n {} -c {} -p {} -port {} {}{}",
        requests, concurrency, postdata, port, base_url, path
    );

    match process::Command::new("bash")
        .arg("-c")
        .arg(cmd.as_str())
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .spawn()
    {
        Ok(child) => Ok(child),
        Err(err) => Err(format!("Can't run your server: {}.", err)),
    }
}
