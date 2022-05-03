use crate::consts::USER1_KEY;
use crate::models::{ListContractByCode, TxResponse, WasmQueryResponse};
use cosmwasm_std::Uint64;
use desmos_bindings::subspaces::query_types::{
    QuerySubspaceResponse, QuerySubspacesResponse, QueryUserGroupMembersResponse,
    QueryUserGroupResponse, QueryUserGroupsResponse,
};
use serde::Serialize;
use serde_json;
use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

pub struct DesmosCli {
    desmos_bin: String,
    desmos_home: String,
    tx_script: String,
}

impl DesmosCli {
    pub fn default() -> DesmosCli {
        DesmosCli::new("../../desmos".to_string())
    }

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

    /// Execute a transaction using the Desmos binary.
    ///
    /// * `args` - Args passed to the desmos `tx` command.
    pub fn execute_tx<I, S>(&self, args: I) -> TxResponse
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let tx_result = DesmosCli::run_command(
            self.tx()
                .args(args)
                // Force output to json so that we can parse the output with serde
                .arg("--output=json")
                .arg("--chain-id=testchain"),
        );

        // Parse the tx response.
        serde_json::from_str(&tx_result).unwrap()
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

        result
            .contracts
            .get(0)
            .expect(&format!("can't find smart contract with id {}", id))
            .to_string()
    }

    /// Execute a smart contract.
    ///
    /// * `contract` - Address of the smart contract to execute.
    /// * `msg` - Message to send to the smart contract.
    pub fn wasm_execute<T>(&self, contract: &str, msg: &T) -> TxResponse
    where
        T: ?Sized + Serialize,
    {
        // Serialize the msg to execute.
        let serialized = serde_json::to_string(msg).unwrap();

        self.execute_tx([
            "wasm",
            "execute",
            contract,
            &serialized,
            &format!("--from={}", USER1_KEY),
        ])
    }

    /// Send a query request to a smart contract
    ///
    /// * `contract` - Smart contract address.
    /// * `query` - Query message to send to the smart contract.
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

    /// Queries all the subspaces created.
    pub fn query_subspaces(&self, page_key: Option<String>) -> QuerySubspacesResponse {
        let mut cmd = self.desmos();
        cmd.args(["query", "subspaces", "subspaces", "-o=json"]);

        if let Some(key) = page_key {
            cmd.arg(format!("--page-key={}", key));
        }

        let result = DesmosCli::run_command(&mut cmd);

        serde_json::from_str(&result).unwrap()
    }

    /// Queries the details of a subspace.
    pub fn query_subspace(&self, subspace_id: Uint64) -> QuerySubspaceResponse {
        let mut cmd = self.desmos();
        cmd.args([
            "query",
            "subspaces",
            "subspace",
            &subspace_id.to_string(),
            "-o=json",
        ]);

        let result = DesmosCli::run_command(&mut cmd);

        serde_json::from_str(&result).unwrap()
    }

    /// Queries the user groups inside a subspace.
    pub fn query_user_groups(
        &self,
        subspace_id: Uint64,
        page_key: Option<String>,
    ) -> QueryUserGroupsResponse {
        let mut cmd = self.desmos();
        cmd.args([
            "query",
            "subspaces",
            "groups",
            "list",
            &subspace_id.to_string(),
            "-o=json",
        ]);

        if let Some(key) = page_key {
            cmd.arg(format!("--page-key={}", key));
        }

        serde_json::from_str(&DesmosCli::run_command(&mut cmd)).unwrap()
    }

    /// Queries the details of a user group inside a subspace.
    pub fn query_user_group(&self, subspace_id: Uint64, group_id: u32) -> QueryUserGroupResponse {
        let mut cmd = self.desmos();
        cmd.args([
            "query",
            "subspaces",
            "groups",
            "group",
            &subspace_id.to_string(),
            &group_id.to_string(),
            "-o=json",
        ]);

        serde_json::from_str(&DesmosCli::run_command(&mut cmd)).unwrap()
    }

    /// Queries the members of a group.
    pub fn query_user_group_members(
        &self,
        subspace_id: Uint64,
        group_id: u32,
        page_key: Option<String>,
    ) -> QueryUserGroupMembersResponse {
        let mut cmd = self.desmos();

        cmd.args([
            "query",
            "subspaces",
            "groups",
            "members",
            &subspace_id.to_string(),
            &group_id.to_string(),
            "-o=json",
        ]);

        if let Some(key) = page_key {
            cmd.arg(format!("--page-key={}", key));
        }

        serde_json::from_str(&DesmosCli::run_command(&mut cmd)).unwrap()
    }
}
