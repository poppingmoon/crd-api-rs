use chrono::NaiveDate;
use derive_builder::Builder;
use serde::{Deserialize, Serialize, Serializer};

use crate::{client::Client, error::Error, response::ResultSet};

/// リクエストパラメータ
///
/// 参照: <https://crd.ndl.go.jp/jp/help/general/api_spec_2.html#reqparam>
///
/// # Example
///
/// ```
/// use anyhow::Result;
///
/// fn main() -> Result<()> {
///     // 質問に rust を含むリファレンス事例を検索するリクエストを作成
///     let request = crd_api::builder()
///         .search_type("reference")
///         .query("question = rust")
///         .build()?;
///     let url = request.url();
///     println!("{url}");
///     // https://crd.ndl.go.jp/api/refsearch?type=reference&query=question+%3D+rust
///
///     Ok(())
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default, Builder)]
pub struct Request {
    /// 検索区分
    ///
    ///    - `reference`: レファレンス事例
    ///    - `manual`: 調べ方マニュアル
    ///    - `collection`: 特別コレクション
    ///    - `profile`: 参加館プロファイル
    ///    - `all`: すべてを対象 (デフォルト)
    ///
    /// `all` の場合, [`query`](Self::query) は `anywhere` のみ使用可能
    #[serde(rename = "type")]
    #[builder(default, setter(strip_option, into))]
    pub search_type: Option<String>,

    /// 検索条件 (いずれか必須)
    ///
    /// CQL方式で各項目に対する検索クエリーを作成する (検索条件の作成に関しては
    /// [CQLフォーマット](https://crd.ndl.go.jp/jp/help/general/api_spec_2.html#cql) 参照)
    #[builder(default, setter(strip_option, into))]
    pub query: Option<String>,

    /// 事例作成日 FROM (いずれか必須)
    #[serde(rename = "crt-date_from", serialize_with = "ser_date_opt")]
    #[builder(default, setter(strip_option, into))]
    pub crt_date_from: Option<NaiveDate>,

    /// 事例作成日 TO (いずれか必須)
    #[serde(rename = "crt-date_to", serialize_with = "ser_date_opt")]
    #[builder(default, setter(strip_option, into))]
    pub crt_date_to: Option<NaiveDate>,

    /// 登録日 FROM (いずれか必須)
    #[serde(rename = "reg-date_from", serialize_with = "ser_date_opt")]
    #[builder(default, setter(strip_option, into))]
    pub reg_date_from: Option<NaiveDate>,

    /// 登録日 TO (いずれか必須)
    #[serde(rename = "reg-date_to", serialize_with = "ser_date_opt")]
    #[builder(default, setter(strip_option, into))]
    pub reg_date_to: Option<NaiveDate>,

    /// 最終更新日 FROM (いずれか必須)
    #[serde(rename = "lst-date_from", serialize_with = "ser_date_opt")]
    #[builder(default, setter(strip_option, into))]
    pub lst_date_from: Option<NaiveDate>,

    /// 最終更新日 TO (いずれか必須)
    #[serde(rename = "lst-date_to", serialize_with = "ser_date_opt")]
    #[builder(default, setter(strip_option, into))]
    pub lst_date_to: Option<NaiveDate>,

    /// 提供館コード

    /// 完全一致で検索する. 提供館名で検索を行いたい場合, [`query`](Self::query) にて指定を行う.
    /// 参加館プロファイルの図書館コードも対象とする
    #[serde(rename = "lib-id")]
    #[builder(default, setter(strip_option, into))]
    pub lib_id: Option<String>,

    /// 検索対象
    ///
    /// - `all`: 全館 (デフォルト)
    /// - `ndl`: 国立国会図書館
    /// - `public`: 公共図書館
    /// - `academic`: 大学図書館
    /// - `special`: 専門図書館
    /// - `school`: 学校図書館
    /// - `archives`: アーカイブズ
    #[serde(rename = "lib-group")]
    #[builder(default, setter(strip_option, into))]
    pub lib_group: Option<String>,

    /// 検索結果取得位置 (デフォルト: 1)
    #[builder(default, setter(strip_option, into))]
    pub results_get_position: Option<i32>,

    /// 検索結果返却件数 (デフォルト: 200)
    #[builder(default, setter(strip_option, into))]
    pub results_num: Option<i32>,

    /// ソート項目
    ///
    /// - レファレンス事例
    ///     - `fit`: 適合度 (デフォルト)
    ///     - `reg-id`: 管理番号
    ///     - `crt-date`: 事例作成日
    ///     - `reg-date`: 登録日時
    ///     - `lst-date`: 最終更新日時
    ///     - `access-num`: アクセス数
    ///     - `applause-num`: 拍手数
    /// - 調べ方マニュアル
    ///     - `fit`: 適合度 (デフォルト)
    ///     - `reg-id`: 管理番号
    ///     - `crt-date`: 調べ方作成日
    ///     - `reg-date`: 登録日時
    ///     - `lst-date`: 最終更新日時
    ///     - `access-num`: アクセス数
    ///     - `applause-num`: 拍手数
    /// - 特別コレクション
    ///     - `fit`: 適合度 (デフォルト)
    ///     - `reg-id`: 管理番号
    ///     - `reg-date`: 登録日時
    ///     - `lst-date`: 最終更新日時
    ///     - `access-num`: アクセス数
    ///     - `applause-num`: 拍手数
    /// - 参加館プロファイル
    ///     - `fit`: 適合度 (デフォルト)
    ///     - `pro-key`: 図書館ヨミ
    ///     - `reg-date`: 登録日時
    ///     - `lst-date`: 最終更新日時
    ///     - `access-num`: アクセス数
    ///     - `applause-num`: 拍手数
    ///
    /// すべて第2ソートキーは最終更新日, 第3ソートキーは登録番号となる
    #[builder(default, setter(strip_option, into))]
    pub sort: Option<String>,

    /// ソート条件
    ///
    /// - `asc`: 昇順
    /// - `desc`: 降順 (デフォルト)
    #[builder(default, setter(strip_option, into))]
    pub sort_order: Option<String>,
}

fn ser_date_opt<S>(date: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(date) = date {
        serializer.serialize_str(&format!("{}", date.format("%Y%m%d")))
    } else {
        serializer.serialize_none()
    }
}

impl Request {
    /// 簡易検索のリクエストを作成する
    pub fn new(search_term: &str) -> Self {
        Self {
            query: Some(format!(r#"anywhere = "{search_term}""#)),
            ..Default::default()
        }
    }

    /// リクエストをクエリストリングに変換する
    pub fn query_string(&self) -> String {
        serde_qs::to_string(self).unwrap()
    }

    /// リクエストURL
    pub fn url(&self) -> String {
        const ENDPOINT: &'static str = "https://crd.ndl.go.jp/api/refsearch";
        let qs = self.query_string();
        format!("{ENDPOINT}?{qs}")
    }

    /// リクエストを行って検索結果を取得する
    ///
    /// # Errors
    ///
    /// 以下の場合エラーを返す
    ///
    /// - リクエストに失敗したとき
    /// - 返却されたXMLの解析に失敗したとき
    /// - APIがエラーを返したとき
    pub async fn search(&self) -> Result<ResultSet, Error> {
        Client::new()?.search(self).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn search_test() {
        let res = RequestBuilder::default()
            .crt_date_from(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap())
            .results_num(1)
            .build()
            .unwrap()
            .search()
            .await;
        res.unwrap();
    }

    #[tokio::test]
    async fn search_example_1() {
        RequestBuilder::default()
            .search_type("reference")
            .query("question any 読書")
            .build()
            .unwrap()
            .search()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn search_example_2() {
        RequestBuilder::default()
            .search_type("reference")
            .results_num(50)
            .query("question any 本 and answer any 村上春樹")
            .build()
            .unwrap()
            .search()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn search_example_3() {
        RequestBuilder::default()
            .search_type("reference")
            .query("question any 本 音楽 and solution = 0")
            .crt_date_from("2000-01-01".parse::<NaiveDate>().unwrap())
            .build()
            .unwrap()
            .search()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn simple_search_test() {
        Request::new("rust").search().await.unwrap();
    }
}
