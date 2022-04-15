use cosmwasm_std::Binary;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TxResponse {
    /// The block height.
    pub height: String,
    /// The transactio hash.
    pub txhash: String,
    /// Response code.
    pub code: u32,
    /// The output of the application's logger.
    pub raw_log: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListContractByCode {
    pub contracts: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmQueryResponse {
    pub data: WasmData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmData {
    pub data: Binary,
}

impl TxResponse {
    /// Tells if the transaction has been performed successfully.
    pub fn is_success(&self) -> bool {
        return self.code == 0;
    }

    pub fn assert_success(&self) {
        if !self.is_success() {
            panic!(
                "Tx failed: {}\nCode: {}\n{}",
                self.txhash, self.code, &self.raw_log
            );
        }
    }
}
