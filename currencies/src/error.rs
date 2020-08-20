use ewf_core::error::Error as EwfError;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Error {
    DatabaseConnectError,
    DatabaseExistsInstallError,
    DatabaseInstallError,
    DatabaseSelectError,
    DatabaseInsertError,
    DatabaseDeleteError,
    DatabaseJsonDeSerializeError,
    CallParamValidFaild,

    CurrencyUnlockError,
    CurrencyByidNotFound,
    CurrencyParamInvalid,
}

impl Error {
    pub fn to_ewf_error(self) -> EwfError {
        match self {
            Error::DatabaseConnectError => EwfError::OtherError("DatabaseConnectError".to_string()),
            Error::DatabaseExistsInstallError => {
                EwfError::OtherError("DatabaseExistsInstallError".to_string())
            }
            Error::DatabaseInstallError => EwfError::OtherError("DatabaseInstallError".to_string()),
            Error::DatabaseSelectError => EwfError::OtherError("DatabaseSelectError".to_string()),
            Error::DatabaseInsertError => EwfError::OtherError("DatabaseInsertError".to_string()),
            Error::DatabaseDeleteError => EwfError::OtherError("DatabaseDeleteError".to_string()),
            Error::DatabaseJsonDeSerializeError => {
                EwfError::OtherError("DatabaseJsonDeSerializeError".to_string())
            }
            Error::CallParamValidFaild => EwfError::CallParamValidFaild,

            Error::CurrencyUnlockError => EwfError::JsonRpcError {
                code: 2001i64,
                msg: "货币解锁失败".to_string(),
            },
            Error::CurrencyByidNotFound => EwfError::JsonRpcError {
                code: 2002i64,
                msg: "指定货币未发现".to_string(),
            },
            Error::CurrencyParamInvalid => EwfError::JsonRpcError {
                code: 2003i64,
                msg: "输入货币未通过校验".to_string(),
            },
        }
    }
}
