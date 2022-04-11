use crate::models::{ListContractByCode, TxResponse, WasmQueryResponse};
use serde::Serialize;
use serde_json;
use std::env;
use std::path::Path;
use std::process::Command;

struct DesmosCli {
    desmos_bin: String,
    desmos_home: String,
    tx_script: String,
}

impl DesmosCli {
    /// Creates a new instance of [`DesmosCli`].
    ///
    /// * `desmos_path` - Path where is stored the desmos binary and the utility scripts.
    pub fn new(desmos_path: String) -> DesmosCli {
        let desmos_bin = format!("{}/desmos", desmos_path);
        let desmos_home = format!("{}/.desmos", desmos_path);
        let tx_script = format!("{}/tx.sh", desmos_path);

        if !Path::new(&desmos_bin).exists() {
            panic!(
                "can't find desmos bin current dir: {}",
                env::current_dir().unwrap().display()
            );
        }

        if !Path::new(&tx_script).exists() {
            panic!(
                "can't find tx.sh bin current dir: {}",
                env::current_dir().unwrap().display()
            );
        }

        DesmosCli {
            desmos_bin,
            desmos_home,
            tx_script,
        }
    }

    /// Returns a [`Command`] to interact with the desmos binary.
    fn desmos(&self) -> Command {
        let mut command = Command::new(&self.desmos_bin);

        // Tell the desmos bin to use our temporary directory.
        command.arg(format!("--home={}", &self.desmos_home));

        command
    }

    /// Returns a [`Command`] to perform transactions.
    fn tx(&self) -> Command {
        Command::new(&self.tx_script)
    }

    /// Runs a desmos command.
    ///
    /// * `cmd` - Command to run.
    pub fn run_command(cmd: &mut Command) -> String {
        let result = cmd.output();

        match result {
            Ok(output) => {
                let stdout =
                    String::from_utf8(output.stdout).expect("error parsing command stdout");
                let stderr =
                    String::from_utf8(output.stderr).expect("error parsing command stderr");

                if output.status.code().unwrap_or(-1) != 0 {
                    panic!("desmos command error {}", stderr);
                }

                return stdout;
            }
            Err(err) => {
                panic!("error: {}", err);
            }
        }
    }

    /// Gets a contract address by it's id.
    ///
    /// * `id` - ID of the contract of interest.
    pub fn get_contract_by_code(&self, id: u64) -> String {
        let mut cmd = self.desmos();

        cmd.arg("query")
            .arg("wasm")
            .arg("list-contract-by-code")
            .arg(id.to_string())
            .arg("--output=json");

        let output = DesmosCli::run_command(&mut cmd);
        let result: ListContractByCode = serde_json::from_str(&output).unwrap();

        result.contracts[0].to_string()
    }

    pub fn wasm_execute<T>(&self, contract: &str, msg: &T) -> TxResponse
    where
        T: ?Sized + Serialize,
    {
        // Get the command to perform transactions
        let mut cmd = self.tx();

        // Serialize the msg to execute.
        let serialized = serde_json::to_string(msg).unwrap();

        // Execute the contract with the provided msg.
        let tx_result = DesmosCli::run_command(cmd.args([
            "wasm",
            "execute",
            contract,
            &serialized,
            "--from=manu",
            "--output=json",
            "--chain-id=testchain",
        ]));

        // Parse the tx response.
        serde_json::from_str(&tx_result).unwrap()
    }

    pub fn wasm_query<T>(&self, contract: &str, query: &T) -> WasmQueryResponse
    where
        T: ?Sized + Serialize,
    {
        let mut cmd = self.desmos();
        let serialized = serde_json::to_string(&query).unwrap();

        let result = DesmosCli::run_command(cmd.args([
            "query",
            "wasm",
            "contract-state",
            "smart",
            contract,
            &serialized,
            "--output=json",
        ]));

        serde_json::from_str(&result).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::chain_communication::DesmosCli;
    use cosmwasm_std::{Addr, QueryRequest};
    use desmos_bindings::profiles::models_profile::Profile;
    use desmos_bindings::profiles::msg::ProfilesMsg::SaveProfile;
    use desmos_bindings::profiles::query::ProfilesQuery;
    use desmos_bindings::query::DesmosQuery;
    use serde_json::Value;
    use test_contract::msg::ExecuteMsg;
    use test_contract::msg::QueryMsg::DesmosChain;

    #[test]
    fn test_create_profile() {
        let desmos_cli = DesmosCli::new("../../desmos".to_string());

        let contract_address = desmos_cli.get_contract_by_code(1);

        let save_profile = SaveProfile {
            dtag: "test_dtag".to_string(),
            nickname: "contract_nick".to_string(),
            bio: "test_bio".to_string(),
            profile_picture: "https://i.imgur.com/X2aK5Bq.jpeg".to_string(),
            cover_picture: "https://i.imgur.com/X2aK5Bq.jpeg".to_string(),
            creator: Addr::unchecked(contract_address.clone()),
        };
        let msg = ExecuteMsg::DesmosMessages {
            msgs: vec![save_profile.into()],
        };

        let tx_result = desmos_cli.wasm_execute(&contract_address, &msg);

        assert!(tx_result.is_success());

        let query_msg = DesmosChain {
            request: QueryRequest::Custom(DesmosQuery::Profiles(ProfilesQuery::Profile {
                user: contract_address.to_string(),
            })),
        };

        let result = desmos_cli.wasm_query(&contract_address, &query_msg);
        assert_eq!("", result.data.data.to_string());
    }

    #[test]
    fn test_query_profile() {
        let desmos_cli = DesmosCli::new("../../desmos".to_string());

        let contract_address = desmos_cli.get_contract_by_code(1);

        let query_msg = DesmosChain {
            request: QueryRequest::Custom(DesmosQuery::Profiles(ProfilesQuery::Profile {
                user: contract_address.to_string(),
            })),
        };

        let result = desmos_cli.wasm_query(&contract_address, &query_msg);
        let decode = base64::decode(result.data.data.to_base64()).unwrap();
        let result = String::from_utf8(decode).unwrap();
        let json: Value = serde_json::from_str(&result).unwrap();
        let profile: Profile =
            serde_json::from_value(json.get("profile").unwrap().clone()).unwrap();
        assert_eq!(profile.dtag, "test_dtag");
        assert_eq!("", result);
    }
}
