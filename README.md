# crd-api-rs

Rust bindings for the Search API 2.0 of the Collaborative Reference Database (CRD)
by the National Diet Library of Japan.

国立国会図書館レファレンス協同データベース (レファ協, CRD) の
[検索用API 2.0](https://crd.ndl.go.jp/jp/help/crds/api.html#chap8-3) を Rust で扱うためのライブラリ

## Examples

```rust
use crd_api::cql::Query;
use crd_api::response::Reference;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 質問に「読書」を含むレファレンス事例を検索
    let request = crd_api::builder()
        .search_type("reference")
        .query(Query::any("question", &["読書"]).to_string())
        .build()?;
    let result = request.search().await?;
    let references: Vec<&Reference> = result.filter_reference().collect();

    Ok(())
}
```
