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

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
