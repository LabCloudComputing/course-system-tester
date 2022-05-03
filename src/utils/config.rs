/*
 * @Author: IceyBlackTea
 * @Date: 2022-03-30 13:11:24
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-05-03 15:31:42
 * @FilePath: /course-system-tester/src/utils/config.rs
 * @Description: Copyright © 2021 IceyBlackTea. All rights reserved.
 */

use std::fs;

pub fn read_config_file(config_file: String) -> Result<serde_json::Value, String> {
    let file = match fs::File::open(config_file) {
        Ok(file) => file,
        Err(err) => {
            return Err(format!("Failed to read tester-config.json: {}", err));
        }
    };

    let config = match serde_json::from_reader(file) {
        Ok(config) => config,
        Err(err) => {
            return Err(format!("Failed to parse config file into JSON: {}", err));
        }
    };

    Ok(config)
}

pub fn get_json_value(config: &serde_json::Value, key: &str) -> Result<serde_json::Value, String> {
    let value = match config.get(key) {
        Some(value) => value,
        None => {
            return Err(format!("Can't find the key '{}' in config file.", key));
        }
    };

    Ok(value.clone())
}

pub fn get_json_value_as_u64(config: &serde_json::Value, key: &str) -> Result<u64, String> {
    let value = match config.get(key) {
        Some(value) => value,
        None => {
            return Err(format!("Config don't have the key '{}'.", key));
        }
    };

    match value.as_u64() {
        Some(value) => Ok(value),
        None => Err(format!(
            "The value of key '{}' is '{}' which should be an unsigned integer.",
            key, value
        )),
    }
}

pub fn get_json_value_as_string(config: &serde_json::Value, key: &str) -> Result<String, String> {
    let value = match config.get(key) {
        Some(value) => value,
        None => {
            return Err(format!("Config don't have the key '{}'.", key));
        }
    };

    match value.as_str() {
        Some(value) => Ok(value.to_string()),
        None => Err(format!(
            "The value of key '{}' is '{}' which should be a string.",
            key, value
        )),
    }
}

pub fn parse_web_server_args(config: &serde_json::Value) -> Result<(String, String), String> {
    let web = get_json_value(config, "web")?;

    let bin = get_json_value_as_string(&web, "bin")?;
    let arguments = get_json_value(&web, "arguments")?;

    let ip = get_json_value_as_string(&arguments, "ip")?;
    let port = get_json_value_as_u64(&arguments, "port")?;
    let config_path = get_json_value_as_string(&arguments, "config_path")?;

    Ok((
        format!(
            "{} --ip {} --port {} --config_path {}",
            bin, ip, port, config_path
        ),
        format!("http://{}:{}", ip, port),
    ))
}

pub fn parse_store_server_args(
    config: &serde_json::Value,
) -> Result<(Vec<Vec<String>>, Vec<Vec<(String, bool)>>), String> {
    let mut cmds_vec = vec![];
    let mut vnics_vec = vec![];
    let stores = get_json_value(config, "stores")?;

    match stores.as_array() {
        Some(stores) => {
            for store in stores {
                let mut cmds = vec![];
                let mut vnics = vec![];

                let bin = get_json_value_as_string(&store, "bin")?;

                let arguments = get_json_value(&store, "arguments")?;

                match arguments.as_array() {
                    Some(arguments) => {
                        for argument in arguments {
                            let argument = argument.as_str().unwrap();
                            cmds.push(format!("{} --config_path {}", bin, argument));
                        }
                    }

                    None => {
                        return Err(format!(
                            "arguments is '{:?}', which should be an array.",
                            arguments
                        ));
                    }
                }

                let vnics_ = get_json_value(&store, "vnics")?;

                match vnics_.as_array() {
                    Some(vnics_) => {
                        for vnic in vnics_ {
                            let vnic = vnic.as_str().unwrap();
                            vnics.push((vnic.to_string(), false));
                        }
                    }

                    None => {
                        return Err(format!(
                            "arguments is '{:?}', which should be an array.",
                            arguments
                        ));
                    }
                }

                cmds_vec.push(cmds);
                vnics_vec.push(vnics);
            }
        }
        None => {
            return Err(format!(
                "stores is '{:?}', which should be an array.",
                stores
            ));
        }
    }

    Ok((cmds_vec, vnics_vec))
}
