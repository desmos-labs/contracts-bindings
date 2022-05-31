use cosmwasm_std::Binary;
use serde::de::DeserializeOwned;
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

impl ToString for WasmQueryResponse {
    fn to_string(&self) -> String {
        let decode = base64::decode(self.data.data.to_base64()).unwrap();
        String::from_utf8(decode).unwrap()
    }
}

impl WasmQueryResponse {
    pub fn to_object<T: DeserializeOwned>(&self) -> T {
        let json_string = self.to_string();
        serde_json::from_str(&json_string).unwrap()
    }
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
