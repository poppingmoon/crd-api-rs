use crate::{
    error::{ApiErrors, Error},
    request::Request,
    response::ResultSet,
};

pub struct Client {
    pub client: reqwest::Client,
}

impl Client {
    pub fn new() -> Result<Self, reqwest::Error> {
        let headers = reqwest::header::HeaderMap::from_iter([(
            reqwest::header::HOST,
            "crd.ndl.go.jp".parse().unwrap(),
        )]);
        Ok(Client {
            client: reqwest::Client::builder()
                .default_headers(headers)
                .user_agent("crd-api-rs")
                .build()?,
        })
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
    pub async fn search(&self, request: &Request) -> Result<ResultSet, Error> {
        let url = request.url();
        let resp = self.client.get(&url).send().await?.text().await?;
        let res = ResultSet::from_xml(&resp);
        if res.is_err() {
            let res = ApiErrors::from_xml(&resp);
            if let Ok(e) = res {
                return Err(e.into());
            }
        }
        res.map_err(Error::De)
    }
}
