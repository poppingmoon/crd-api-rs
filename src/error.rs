use std::fmt::Display;

use quick_xml::DeError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
#[error(transparent)]
pub enum Error {
    Request(#[from] reqwest::Error),
    De(#[from] DeError),
    Api(#[from] ApiErrors),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
struct ErrResultSet {
    results_cd: u8,
    err_list: ErrList,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
struct ErrList {
    err_item: Vec<ApiError>,
}

/// エラー情報リストノード
#[derive(Error, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename = "err_item")]
pub struct ApiErrors(Vec<ApiError>);

impl ApiErrors {
    pub fn from_xml(s: &str) -> Result<Self, quick_xml::DeError> {
        quick_xml::de::from_str(s)
    }
}

impl Display for ApiErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .0
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{s}")
    }
}

impl<'de> Deserialize<'de> for ApiErrors {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let result_set = ErrResultSet::deserialize(deserializer)?;
        Ok(ApiErrors(result_set.err_list.err_item))
    }
}

/// エラー情報ノード
#[derive(Error, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[error("{err_msg}")]
pub struct ApiError {
    /// エラーコード
    pub err_code: String,

    /// エラーフィールド
    ///
    /// エラーが発生したフィールド (検索リクエストのパラメタ) を表示する
    /// - `query` パラメタ内でのエラーの場合, 該当の検索キーが出力される
    pub err_fld: String,

    /// エラーメッセージ
    pub err_msg: String,
}

#[cfg(test)]
mod tests {
    use quick_xml::de::from_str;

    use super::*;

    #[test]
    fn api_errors_test() {
        let s = "<result_set>
        <results_cd>1</results_cd>
        <err_list>
            <err_item>
                <err_code>0101</err_code>
                <err_fld/>
                <err_msg>検索必須項目が指定されていません。</err_msg>
            </err_item>
            <err_item>
                <err_code>0503</err_code>
                <err_fld>ndc</err_fld>
                <err_msg>【ndc】に使用できない値が指定されています。</err_msg>
            </err_item>
        </err_list>
        </result_set>";
        let e: ApiErrors = from_str(s).unwrap();
        assert_eq!(e.0.len(), 2);
        assert_eq!(e.0[0].err_code, "0101");
        assert_eq!(e.0[1].err_code, "0503");
    }

    #[test]
    fn api_error_test() {
        let s = "<err_item>
        <err_code>0101</err_code>
        <err_fld/>
        <err_msg>検索必須項目が指定されていません。</err_msg>
        </err_item>";
        let e: ApiError = from_str(s).unwrap();
        assert_eq!(e.err_code, "0101");
        assert_eq!(e.err_fld, "");
        assert_eq!(e.err_msg, "検索必須項目が指定されていません。");
    }
}
