use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// CQLフォーマットの検索クエリー
///
/// 参照: `<https://crd.ndl.go.jp/jp/help/general/api_spec_2.html#cql>`
///
/// # Example
/// ```
/// use crd_api::cql::Query;
///
/// // 任意の項目に "rust" と "language" の両方を含む要素を指定するクエリー
/// let query = Query::all("anywhere", &["rust", "language"]);
/// let cql = query.to_string();
/// println!("{cql}")
/// // anywhere all rust language
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Query {
    /// 単一の検索句
    SearchClause {
        index: Index,
        relation: Relation,
        search_term: Vec<String>,
    },

    /// 複数の検索句の結合
    ScopedClause {
        left: Box<Query>,
        boolean: Boolean,
        right: Box<Query>,
    },
}

impl Query {
    /// 簡易検索
    pub fn new(search_term: &[&str]) -> Self {
        Self::equal("anywhere", search_term)
    }

    /// 複数のキーワードをAND演算で検索する
    pub fn all(index: impl Into<Index>, search_term: &[&str]) -> Self {
        Self::SearchClause {
            index: index.into(),
            relation: Relation::All,
            search_term: search_term.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// 複数のキーワードをOR演算で検索する
    pub fn any(index: impl Into<Index>, search_term: &[&str]) -> Self {
        Self::SearchClause {
            index: index.into(),
            relation: Relation::Any,
            search_term: search_term.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// 指定キーワードでの一致検索
    pub fn equal(index: impl Into<Index>, search_term: &[&str]) -> Self {
        Self::SearchClause {
            index: index.into(),
            relation: Relation::Equal,
            search_term: search_term.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// 2つの検索句をAND条件で結合する
    pub fn and(self, right: Self) -> Self {
        Self::ScopedClause {
            left: self.into(),
            boolean: Boolean::And,
            right: right.into(),
        }
    }

    /// 2つの検索句をOR条件で結合する
    pub fn or(self, right: Self) -> Self {
        Self::ScopedClause {
            left: self.into(),
            boolean: Boolean::Or,
            right: right.into(),
        }
    }

    /// 第1検索句の条件に一致するものから, 第2検索句に該当するものを除外
    pub fn not(self, right: Self) -> Self {
        Self::ScopedClause {
            left: self.into(),
            boolean: Boolean::Not,
            right: right.into(),
        }
    }
}

impl Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ScopedClause {
                left,
                boolean,
                right,
            } => {
                write!(f, "{left} {boolean} ")?;
                match **right {
                    Self::ScopedClause { .. } => write!(f, "( {right} )")?,
                    Self::SearchClause { .. } => write!(f, "{right}")?,
                };
            }
            Self::SearchClause {
                index,
                relation,
                search_term,
            } => {
                let search_term = search_term.join(" ");
                write!(f, "{index} {relation} {search_term}")?;
            }
        }
        Ok(())
    }
}

/// クエリー対象項目
///
/// 参照: `<https://crd.ndl.go.jp/jp/help/general/api_spec_2.html#cql>`
///
/// - レファレンス事例
///     - `anywhere`: 全項目 (簡易検索)
///         - 簡易検索と同範囲での検索となる
///     - `question`: 質問
///     - `reg-id`: 管理番号
///         - 前方一致
///     - `answer`: 回答
///     - `solution`: 解決／未解決
///         - 完全一致
///             - `0`: 解決
///             - `1`: 未解決
///             - `resolved`: 解決
///             - `unresolved`: 未解決
///     - `keyword`: キーワード
///     - `ndc`: NDC
///         - 前方一致
///     - `res-type`: 調査種別
///     - `con-type`: 内容種別
///     - `bibl-desc`: 参考資料 (書誌的事項等)
///         - 参考資料の書誌的事項と備考を検索する
///     - `bibl-isbn`: 参考資料 (ISBN)
///     - `ans-proc`: 回答プロセス
///     - `referral`: 照会先
///     - `pre-res`: 事前調査事項
///     - `note`: 備考
///     - `ptn-type`: 質問者区分
///     - `contri`: 寄与者
///     - `sys-id`: 登録番号
///         - 完全一致
///     - `lib-name`: 提供館名
/// - 調べ方マニュアル
///     - `anywhere`: 全項目 (簡易検索)
///         - 簡易検索と同範囲での検索となる
///     - `theme`: 調査テーマ
///     - `reg-id`: 管理番号
///         - 前方一致
///     - `guide`: 調べ方
///     - `completion`: 完成／未完成
///         - 完全一致
///             - `0`: 完成
///             - `1`: 未完成
///             - `complete`: 完成
///             - `incomplete`: 未完成
///     - `keyword`: キーワード
///     - `ndc`: NDC
///         - 前方一致
///     - `bibl-desc`: 参考資料 (書誌的事項等)
///         - 参考資料の書誌的事項と備考を検索する
///     - `bibl-isbn`: 参考資料 (ISBN)
///     - `note`: 備考
///     - `sys-id`: 登録番号
///         - 完全一致
///     - `lib-name`: 提供館名
/// - 特別コレクション
///     - `anywhere`: 全項目 (簡易検索)
///         - 簡易検索と同範囲での検索となる。
///     - `col-name`: コレクション名
///         - コレクション名、コレクション名ヨミを検索する
///     - `reg-id`: 管理番号
///         - 前方一致
///     - `outline`: 内容
///     - `origin`: 来歴
///     - `restriction`: 利用条件
///     - `catalog`: 目録等
///     - `literature`: 紹介文献
///     - `number`: 所蔵点数
///     - `continue`: 継続
///         - 完全一致
///             - `0`: 継続有
///             - `1`: 継続無
///             - `continue`: 継続有
///             - `discontinued`: 継続無
///     - `keyword`: キーワード
///     - `ndc`: NDC
///         - 前方一致
///     - `note`: 備考
///     - `sys-id`: 登録番号
///         - 完全一致
///     - `lib-name`: 提供館名
/// - 参加館プロファイル
///     - `anywhere`: 全項目 (簡易検索)
///         - 簡易検索と同範囲での検索となる
///     - `lib-type`: 館種
///         - 完全一致
///         - 下記コード値、デコード値ともに許可する
///             - `11`: 国立国会図書館(東京本館)
///             - `12`: 国立国会図書館(関西館)
///             - `13`: 国立国会図書館(国際子ども図書館)
///             - `14`: 国立国会図書館(支部図書館)
///             - `21`: 公共図書館(都道府県立)
///             - `22`: 公共図書館(政令都市立)
///             - `23`: 公共図書館(市立・特別区立)
///             - `24`: 公共図書館(町村立)
///             - `31`: 大学図書館(国立大学)
///             - `32`: 大学図書館(公立大学)
///             - `33`: 大学図書館(私立大学)
///             - `35`: 大学図書館(高等専門)
///             - `41`: 専門図書館(国公立)
///             - `42`: 専門図書館(公益法人)
///             - `43`: 専門図書館(企業)
///             - `44`: 専門図書館(その他)
///             - `51`: 学校図書館(高等学校)
///             - `52`: 学校図書館(中学校)
///             - `53`: 学校図書館(小学校)
///             - `54`: 学校図書館(その他)
///             - `90`: アーカイブズ
///     - `lib-name`: 図書館名
///         - 以下を検索する
///             - 図書館名 (正式)
///             - 図書館名 (略式)
///             - 図書館名ヨミ
///     - `address`: 住所
///         - 以下を検索する
///             - 住所 (都道府県)
///             - 住所 (市区町村)
///             - 住所 (丁目・番地)
///             - (= 住所 (検索用))
///     - `open-info`: 開館情報
///     - `restriction`: 利用条件
///     - `outline`: 沿革
///     - `feature`: 特色
///     - `notes`: 注意事項
///     - `access`: 交通アクセス
///     - `isil`: ISIL
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Index(String);

impl Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Index {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Relation {
    /// 複数のキーワードをAND演算で検索する
    All,

    /// 複数のキーワードをOR演算で検索する
    Any,

    /// 指定キーワードでの一致検索
    Equal,
}

impl Display for Relation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Any => write!(f, "any"),
            Self::Equal => write!(f, "="),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Boolean {
    /// 2つの検索句をAND条件で結合する
    And,

    /// 2つの検索句をOR条件で結合する
    Or,

    /// 第1検索句の条件に一致するものから, 第2検索句に該当するものを除外。
    Not,
}

impl Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
            Self::Not => write!(f, "not"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cql_test() {
        let q1 = Query::SearchClause {
            index: "question".into(),
            relation: Relation::Equal,
            search_term: vec!["rust".to_string()],
        };
        let q2 = Query::SearchClause {
            index: "answer".into(),
            relation: Relation::Any,
            search_term: vec!["programming".to_string(), "language".to_string()],
        };
        let q = Query::ScopedClause {
            left: q1.into(),
            boolean: Boolean::And,
            right: q2.into(),
        };
        let s: String = q.to_string();
        assert_eq!(s, "question = rust and answer any programming language")
    }

    #[test]
    fn cql_test2() {
        let q = Query::any("question", &["本"]).and(Query::any("answer", &["村上春樹"]));
        assert_eq!(q.to_string(), "question any 本 and answer any 村上春樹")
    }

    #[test]
    fn cql_test3() {
        let q1 = Query::any("question", &["本", "音楽"])
            .and(Query::equal("solution", &["resolved"]))
            .or(Query::equal("ptn-type", &["学生"]));
        let q2 = Query::ScopedClause {
            left: Query::ScopedClause {
                left: Query::SearchClause {
                    index: "question".into(),
                    relation: Relation::Any,
                    search_term: vec!["本".to_string(), "音楽".to_string()],
                }
                .into(),
                boolean: Boolean::And,
                right: Query::SearchClause {
                    index: "solution".into(),
                    relation: Relation::Equal,
                    search_term: vec!["resolved".to_string()],
                }
                .into(),
            }
            .into(),
            boolean: Boolean::Or,
            right: Query::SearchClause {
                index: "ptn-type".into(),
                relation: Relation::Equal,
                search_term: vec!["学生".to_string()],
            }
            .into(),
        };
        assert_eq!(q1, q2);
    }
}
