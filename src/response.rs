use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Deserializer, Serialize};

/// 返却結果ルートノード
///
/// 参照: `<https://crd.ndl.go.jp/jp/help/general/api_spec_2.html#response>`
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct ResultSet {
    /// ヒット数
    ///
    /// リクエストされた検索条件に該当する事例数（ヒット数）
    pub hit_num: u32,

    /// 検索開始位置
    ///
    /// - リクエストで指定された [`results_get_position`](crate::request::Request::results_get_position) の値と同一の値を出力
    /// - リクエストで指定されなかった場合は `1` (デフォルト: 先頭位置から検索開始) を出力
    pub results_get_position: u32,

    /// 検索結果返却件数
    pub results_num: u32,

    /// 処理結果コード
    results_cd: u32,

    /// 返却結果フィールド
    pub result: Vec<ResultItem>,
}

impl ResultSet {
    /// xml形式の文字列から [`ResultSet`] に変換する
    ///
    /// # Errors
    ///
    /// xmlの解析に失敗したときエラーを返す
    pub fn from_xml(s: &str) -> Result<Self, quick_xml::DeError> {
        quick_xml::de::from_str(s)
    }

    /// 結果の要素数を返す
    pub fn len(&self) -> usize {
        self.result.len()
    }

    /// 結果の要素のイテレータを返す
    pub fn iter(&self) -> impl Iterator<Item = &ResultItem> {
        self.result.iter()
    }

    /// 結果の内のレファレンス事例のイテレータを返す
    pub fn filter_reference(&self) -> impl Iterator<Item = &Reference> {
        self.iter().filter_map(|i| {
            if let ResultItem::Reference(r) = i {
                Some(r)
            } else {
                None
            }
        })
    }

    /// 結果の内の調べ方マニュアルのイテレータを返す
    pub fn filter_manual(&self) -> impl Iterator<Item = &Manual> {
        self.iter().filter_map(|i| {
            if let ResultItem::Manual(m) = i {
                Some(m)
            } else {
                None
            }
        })
    }

    /// 結果の内の特別コレクションのイテレータを返す
    pub fn filter_collection(&self) -> impl Iterator<Item = &Collection> {
        self.iter().filter_map(|i| {
            if let ResultItem::Collection(c) = i {
                Some(c)
            } else {
                None
            }
        })
    }

    /// 結果の内の参加館プロファイルのイテレータを返す
    pub fn filter_profile(&self) -> impl Iterator<Item = &Profile> {
        self.iter().filter_map(|i| {
            if let ResultItem::Profile(p) = i {
                Some(p)
            } else {
                None
            }
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
struct ResultItemWrapper {
    reference: Option<Reference>,
    manual: Option<Manual>,
    collection: Option<Collection>,
    profile: Option<Profile>,
}

/// 返却結果フィールド
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub enum ResultItem {
    /// レファレンス事例
    Reference(Reference),

    /// 調べ方マニュアル
    Manual(Manual),

    /// 特別コレクション
    Collection(Collection),

    /// 参加館プロファイル
    Profile(Profile),
}

impl<'de> Deserialize<'de> for ResultItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let result = ResultItemWrapper::deserialize(deserializer)?;
        if let Some(reference) = result.reference {
            Ok(ResultItem::Reference(reference))
        } else if let Some(manual) = result.manual {
            Ok(ResultItem::Manual(manual))
        } else if let Some(collection) = result.collection {
            Ok(ResultItem::Collection(collection))
        } else if let Some(profile) = result.profile {
            Ok(ResultItem::Profile(profile))
        } else {
            Err(serde::de::Error::custom(
                "missing field, expected one of `reference`, `manual`, `collection`, `profile`",
            ))
        }
    }
}

/// レファレンス事例
///
/// 参照: `<https://crd.ndl.go.jp/jp/help/general/api_xmlfmt.html#api_xmlfmt_ref>`
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Reference {
    /// 質問
    pub question: String,

    /// 管理番号
    pub reg_id: String,

    /// 回答
    pub answer: String,

    /// 事例作成日
    ///
    /// 事例作成日が入力されていない, またはYYYYMMDD形式に変換できない場合は [`None`]
    #[serde(deserialize_with = "de_date_opt", default)]
    pub crt_date: Option<NaiveDate>,

    /// 解決／未解決
    ///
    /// 解決なら [`true`]
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "de_bool_opt",
        default
    )]
    pub solution: Option<bool>,

    /// キーワード
    pub keyword: Option<Vec<String>>,

    /// 分類
    pub class: Option<Vec<Class>>,

    /// 調査種別
    ///
    /// 「文献紹介」「事実調査」「書誌的事項調査」「所蔵調査」「所蔵機関調査」「利用案内」「その他」または任意の文字列
    pub res_type: Option<String>,

    /// 内容種別
    ///
    /// 「郷土」「人物」「言葉」「地名」または任意の文字列
    pub con_type: Option<String>,

    /// 参考資料
    pub bibl: Option<Vec<Bibl>>,

    /// 回答プロセス
    pub ans_proc: Option<String>,

    /// 照会先
    pub referral: Option<Vec<String>>,

    /// 事前調査事項
    pub pre_res: Option<String>,

    /// 備考
    pub note: Option<String>,

    /// 質問者区分
    ///
    /// 「未就学児」「小中学生」「高校生」「学生」「社会人」「団体」「図書館」または任意の文字列
    pub ptn_type: Option<String>,

    /// 寄与者
    pub contri: Option<Vec<String>>,

    /// システム管理項目
    pub system: System,

    /// URL
    ///
    /// 一般公開用詳細表示画面のURL
    pub url: String,
}

/// 調べ方マニュアル
///
/// 参照: `<https://crd.ndl.go.jp/jp/help/general/api_xmlfmt.html#api_xmlfmt_man>`
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Manual {
    /// 調査テーマ
    pub theme: String,

    /// 管理番号
    pub reg_id: String,

    /// 調べ方
    pub guide: String,

    /// 調べ方作成日
    ///
    /// 事例作成日が入力されていない, またはYYYYMMDD形式に変換できない場合は [`None`]
    #[serde(deserialize_with = "de_date_opt", default)]
    pub crt_date: Option<NaiveDate>,

    /// 完成／未完成
    ///
    /// 完成なら [`true`]
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "de_completion",
        default
    )]
    pub completion: Option<bool>,

    /// キーワード
    pub keyword: Option<Vec<String>>,

    /// 分類
    pub class: Option<Vec<Class>>,

    /// 参考資料
    pub bibl: Option<Vec<Bibl>>,

    /// 備考
    pub note: Option<String>,

    // システム管理項目
    pub system: System,

    /// URL
    ///
    /// 一般公開用詳細表示画面のURL
    pub url: String,
}

/// 特別コレクション
///
/// 参照: `<https://crd.ndl.go.jp/jp/help/general/api_xmlfmt.html#api_xmlfmt_col>`
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Collection {
    /// コレクション名
    pub col_name: String,

    /// コレクション名ヨミ
    pub pro_key: String,

    /// 管理番号
    pub reg_id: String,

    /// 内容
    pub outline: String,

    /// 来歴
    pub origin: Option<String>,

    /// 利用条件
    pub restriction: Option<String>,

    /// 目録等
    pub catalog: Option<String>,

    /// 紹介文献
    pub literature: Option<String>,

    /// 所蔵点数
    pub number: Option<String>,

    /// 継続
    ///
    /// 継続有なら [`true`]
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "continue",
        deserialize_with = "de_bool_opt",
        default
    )]
    pub collection_continue: Option<bool>,

    /// キーワード
    pub keyword: Option<Vec<String>>,

    /// 分類
    pub class: Option<Vec<Class>>,

    /// 備考
    pub note: Option<String>,

    /// システム
    pub system: System,

    /// URL
    ///
    /// 一般公開用詳細表示画面のURL
    pub url: String,
}

/// 参加館プロファイル
///
/// 参照: `<https://crd.ndl.go.jp/jp/help/general/api_xmlfmt.html#api_xmlfmt_pro>`
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Profile {
    /// 館種コード
    pub lib_type: String,

    /// 図書館名（正式）
    pub lib_name: String,

    /// 図書館名（略式）
    pub abbr: String,

    /// 図書館名ヨミ
    pub pro_key: String,

    /// 郵便番号
    pub zip_code: String,

    /// 住所 (都道府県)
    pub add_pref: String,

    /// 住所 (市区町村)
    pub add_city: String,

    /// 住所 (丁目・番地)
    pub add_street: String,

    /// 電話番号1
    pub tel1: String,

    /// 電話番号1 (追加情報)
    pub tel1_note: Option<String>,

    /// 電話番号2
    pub tel2: Option<String>,

    /// 電話番号2 (追加情報)
    pub tel2_note: Option<String>,

    /// 電話番号3
    pub tel3: Option<String>,

    /// 電話番号3 (追加情報)
    pub tel3_note: Option<String>,

    /// FAX番号
    pub fax: Option<String>,

    /// E-MAIL (公開)
    pub e_mail: Option<String>,

    /// URL
    pub lib_url: Option<String>,

    /// 開館情報
    pub open_info: Option<String>,

    /// 利用条件
    pub restriction: Option<String>,

    /// 沿革
    pub outline: Option<String>,

    /// 特色
    pub feature: Option<String>,

    /// 注意事項
    pub notes: Option<String>,

    /// 交通アクセス
    pub access: Option<String>,

    /// ISIL
    ///
    /// 複数番号が連結して入力されている場合がある
    pub isil: Option<String>,

    /// システム管理項目
    pub system: LibSystem,

    /// URL
    ///
    /// 一般公開用詳細表示画面のURL
    pub url: String,
}

/// 分類
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Class {
    /// 分類の種類 (NDCのみ)
    #[serde(rename = "@type")]
    pub class_type: String,

    /// 分類のバージョン
    ///
    /// 例: 9 (9版を示す)
    #[serde(rename = "@version")]
    pub version: Option<String>,

    /// 分類の番号
    #[serde(rename = "$value")]
    pub class: String,
}

/// 参考資料
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct Bibl {
    /// 書誌的事項
    pub bibl_desc: Option<String>,

    /// ISBN
    pub bibl_isbn: Option<String>,

    /// 備考
    pub bibl_note: Option<String>,
}

/// システム管理項目
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct System {
    /// 登録日時
    #[serde(deserialize_with = "de_datetime")]
    pub reg_date: NaiveDateTime,

    /// 最終更新日時
    #[serde(deserialize_with = "de_datetime")]
    pub lst_date: NaiveDateTime,

    /// システムID (登録番号)
    ///
    /// 一意となるキー
    pub sys_id: String,

    /// 提供館コード
    pub lib_id: String,

    /// 提供館名
    pub lib_name: String,

    /// 関連ファイル数
    ///
    /// 関連ファイルの登録がある場合は登録数を表示する.
    /// 登録がない場合は `0` を返却する
    pub file_num: u32,
}

/// システム管理項目 (参加館プロファイル)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct LibSystem {
    /// 登録日時
    #[serde(deserialize_with = "de_datetime")]
    pub reg_date: NaiveDateTime,

    /// 最終更新日時
    #[serde(deserialize_with = "de_datetime")]
    pub lst_date: NaiveDateTime,

    /// 図書館コード
    ///
    /// 一意となるキー
    pub lib_id: String,

    /// 図書館名（正式）
    pub lib_name: String,

    /// 関連ファイル数
    ///
    /// 関連ファイルの登録がある場合は登録数を表示する.
    /// 登録がない場合は `0` を返却する
    pub file_num: u32,
}

fn de_bool_opt<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s == "0" {
        Ok(Some(true))
    } else if s == "1" {
        Ok(Some(false))
    } else {
        Err(serde::de::Error::custom(format!(
            "failed to parse `{s}` to bool"
        )))
    }
}

fn de_completion<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s == "1" {
        Ok(Some(true))
    } else if s == "2" {
        Ok(Some(false))
    } else {
        Err(serde::de::Error::custom(format!(
            "failed to parse `{s}` to bool"
        )))
    }
}

fn de_date_opt<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s == "00000000" {
        Ok(None)
    } else {
        NaiveDate::parse_from_str(&s, "%Y%m%d")
            .map_err(serde::de::Error::custom)
            .map(Some)
    }
}

fn de_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y%m%d%H%M%S").map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn result_set_test() {
        let s = r#"<?xml version="1.0" encoding="UTF-8"?>
        <result_set>
            <hit_num>123</hit_num>
            <results_get_position>1</results_get_position>
            <results_num>3</results_num>
            <results_cd>0</results_cd>
            <result>
                <reference>
                    <question>質問1</question>
                    <reg-id>001</reg-id>
                    <answer>回答1</answer>
                    <crt-date>20321020</crt-date>
                    <solution>1</solution>
                    <keyword>キーワード1-1</keyword>
                    <keyword>キーワード1-2</keyword>
                    <keyword>キーワード1-3</keyword>
                    <class type="NDC" version="9">21</class>
                    <res-type>事実調査</res-type>
                    <con-type>郷土</con-type>
                    <bibl>
                        <bibl-desc>参考資料1-1</bibl-desc>
                    </bibl>
                    <bibl>
                        <bibl-desc>参考資料1-2</bibl-desc>
                    </bibl>
                    <ans-proc>回答プロセス1</ans-proc>
                    <ptn-type>社会人</ptn-type>
                    <system>
                        <reg-date>20321101115753</reg-date>
                        <lst-date>20330222171423</lst-date>
                        <sys-id>1100323256</sys-id>
                        <lib-id>6210033</lib-id>
                        <lib-name>図書館</lib-name>
                        <file-num>0</file-num>
                    </system>
                    <url>https://crd.ndl.go.jp/reference/detail?page=ref_view&amp;id=1100323256</url>
                </reference>
            </result>
            <result>
                <reference>
                    <question>質問2</question>
                    <reg-id>002</reg-id>
                    <answer>回答2</answer>
                    <crt-date>20320813</crt-date>
                    <solution>0</solution>
                    <keyword>キーワード2-1</keyword>
                    <keyword>キーワード2-2</keyword>
                    <keyword>キーワード2-3</keyword>
                    <keyword>キーワード2-4</keyword>
                    <keyword>キーワード2-5</keyword>
                    <keyword>キーワード2-6</keyword>
                    <class type="NDC" version="9">213</class>
                    <class type="NDC" version="9">681</class>
                    <res-type>文献紹介</res-type>
                    <con-type>郷土</con-type>
                    <bibl>
                        <bibl-desc>参考資料2-1</bibl-desc>
                    </bibl>
                    <bibl>
                        <bibl-desc>参考資料2-2</bibl-desc>
                    </bibl>
                    <bibl>
                        <bibl-desc>参考資料2-3</bibl-desc>
                    </bibl>
                    <bibl>
                        <bibl-desc>参考資料2-4</bibl-desc>
                    </bibl>
                    <ans-proc>回答プロセス2</ans-proc>
                    <ptn-type>社会人</ptn-type>
                    <system>
                        <reg-date>20320911100445</reg-date>
                        <lst-date>20330222165442</lst-date>
                        <sys-id>1100321105</sys-id>
                        <lib-id>6210033</lib-id>
                        <lib-name>図書館</lib-name>
                        <file-num>0</file-num>
                    </system>
                    <url>https://crd.ndl.go.jp/reference/detail?page=ref_view&amp;id=1100321105</url>
                </reference>
            </result>
            <result>
                <reference>
                    <question>質問3</question>
                    <reg-id>003</reg-id>
                    <answer>回答3</answer>
                    <crt-date>20321001</crt-date>
                    <solution>0</solution>
                    <keyword>キーワード3</keyword>
                    <class type="NDC" version="9">182</class>
                    <class type="NDC" version="9">291</class>
                    <res-type>文献紹介</res-type>
                    <con-type>人物</con-type>
                    <bibl>
                        <bibl-desc>参考資料3-1</bibl-desc>
                    </bibl>
                    <bibl>
                        <bibl-desc>参考資料3-2</bibl-desc>
                    </bibl>
                    <ans-proc>回答プロセス3</ans-proc>
                    <ptn-type>社会人</ptn-type>
                    <system>
                        <reg-date>20321005155303</reg-date>
                        <lst-date>20330222165412</lst-date>
                        <sys-id>1100322309</sys-id>
                        <lib-id>6210033</lib-id>
                        <lib-name>図書館</lib-name>
                        <file-num>0</file-num>
                    </system>
                    <url>https://crd.ndl.go.jp/reference/detail?page=ref_view&amp;id=1100322309</url>
                </reference>
            </result>
        </result_set>
        "#;
        let result_set: ResultSet = from_str(s).unwrap();
        assert_eq!(result_set.hit_num, 123);
        assert_eq!(result_set.results_get_position, 1);
        assert_eq!(result_set.results_num, 3);
        assert_eq!(result_set.result.len(), 3);
        let refs: Vec<&Reference> = result_set.filter_reference().collect();
        assert_eq!(refs[0].question, "質問1");
        assert_eq!(refs[1].reg_id, "002");
        assert_eq!(refs[2].answer, "回答3");
    }

    #[test]
    fn reference_test() {
        let reference = r#"<reference>
        <question>質問</question>
        <reg-id>管理番号001</reg-id>
        <answer>回答</answer>
        <crt-date>20321213</crt-date>
        <solution>0</solution>
        <keyword>キーワード1</keyword>
        <keyword>キーワード2</keyword>
        <keyword>キーワード3</keyword>
        <class type="NDC" version="9">913</class>
        <res-type>文献紹介</res-type>
        <con-type>一般</con-type>
        <bibl>
            <bibl-desc>参考資料</bibl-desc>
            <bibl-isbn>9794093865821</bibl-isbn>
        </bibl>
        <ans-proc>回答プロセス</ans-proc>
        <referral>図書館1</referral>
        <referral>図書館2</referral>
        <referral>図書館3</referral>
        <pre-res>事前調査</pre-res>
        <note>備考</note>
        <ptn-type>60代～</ptn-type>
        <contri>寄与者</contri>
        <system>
            <reg-date>20330126003002</reg-date>
            <lst-date>20330222115318</lst-date>
            <sys-id>1100327950</sys-id>
            <lib-id>6310085</lib-id>
            <lib-name>図書館</lib-name>
            <file-num>0</file-num>
        </system>
        <url>https://crd.ndl.go.jp/reference/detail?page=ref_view&amp;id=1100327950</url>
        </reference>"#;
        let reference: Reference = from_str(reference).unwrap();

        assert_eq!(reference.question, "質問");
        assert_eq!(reference.reg_id, "管理番号001");
        assert_eq!(reference.answer, "回答");
        assert_eq!(
            reference.crt_date.unwrap(),
            NaiveDate::from_ymd_opt(2032, 12, 13).unwrap()
        );
        assert_eq!(reference.solution.unwrap(), true);
        assert_eq!(
            reference.keyword.unwrap(),
            ["キーワード1", "キーワード2", "キーワード3"]
        );
        assert_eq!(reference.class.unwrap()[0].class, "913");
        assert_eq!(reference.res_type.unwrap(), "文献紹介");
        assert_eq!(reference.con_type.unwrap(), "一般");
        assert_eq!(
            reference.bibl.as_ref().unwrap()[0]
                .bibl_desc
                .as_ref()
                .unwrap(),
            "参考資料"
        );
        assert_eq!(
            reference.bibl.as_ref().unwrap()[0]
                .bibl_isbn
                .as_ref()
                .unwrap(),
            "9794093865821"
        );
        assert_eq!(reference.bibl.unwrap()[0].bibl_note, None);
        assert_eq!(reference.ans_proc.unwrap(), "回答プロセス");
        assert_eq!(
            reference.referral.unwrap(),
            ["図書館1", "図書館2", "図書館3"]
        );
        assert_eq!(reference.pre_res.unwrap(), "事前調査");
        assert_eq!(reference.note.unwrap(), "備考");
        assert_eq!(reference.ptn_type.unwrap(), "60代～");
        assert_eq!(reference.contri.unwrap(), ["寄与者"]);
        assert_eq!(reference.system.sys_id, "1100327950");
        assert_eq!(reference.system.lib_id, "6310085");
        assert_eq!(reference.system.lib_name, "図書館");
        assert_eq!(reference.system.file_num, 0);
        assert_eq!(
            reference.url,
            "https://crd.ndl.go.jp/reference/detail?page=ref_view&id=1100327950"
        );
    }

    #[test]
    fn manual_test() {
        let manual = r#"<manual>
        <theme>テーマ</theme>
        <reg-id>2032R006</reg-id>
        <guide>調べ方</guide>
        <crt-date>20330213</crt-date>
        <completion>2</completion>
        <class type="NDC">219</class>
        <bibl>
            <bibl-desc>参考資料1</bibl-desc>
            <bibl-note>当館所蔵</bibl-note>
        </bibl>
        <bibl>
            <bibl-desc>参考資料2</bibl-desc>
            <bibl-note>当館所蔵</bibl-note>
        </bibl>
        <note>貸出</note>
        <system>
            <reg-date>20330213170856</reg-date>
            <lst-date>20330220110042</lst-date>
            <sys-id>2100028171</sys-id>
            <lib-id>6110045</lib-id>
            <lib-name>附属図書館</lib-name>
            <file-num>0</file-num>
        </system>
        <url>https://crd.ndl.go.jp/reference/detail?page=man_view&amp;id=2100028171</url>
        </manual>"#;
        let manual: Manual = from_str(manual).unwrap();
        assert_eq!(manual.theme, "テーマ");
        assert_eq!(manual.reg_id, "2032R006");
        assert_eq!(manual.guide, "調べ方");
        assert_eq!(
            manual.crt_date.unwrap(),
            NaiveDate::from_ymd_opt(2033, 2, 13).unwrap()
        );
        assert_eq!(manual.completion.unwrap(), false);
        assert_eq!(manual.keyword, None);
        assert_eq!(manual.class.unwrap()[0].class, "219");
        assert_eq!(
            manual.bibl.unwrap()[0].bibl_desc.as_ref().unwrap(),
            "参考資料1"
        );
        assert_eq!(manual.note.unwrap(), "貸出");
        assert_eq!(manual.system.lib_name, "附属図書館");
        assert_eq!(
            manual.url,
            "https://crd.ndl.go.jp/reference/detail?page=man_view&id=2100028171"
        );
    }

    #[test]
    fn collection_test() {
        let collection = r#"<collection>
        <col-name>地図</col-name>
        <pro-key>チズ</pro-key>
        <reg-id>0000-000</reg-id>
        <outline>内容</outline>
        <restriction>要図書館カード</restriction>
        <catalog>蔵書検索にて一覧表示が可能</catalog>
        <literature>ホームページ</literature>
        <number>75点</number>
        <continue>1</continue>
        <keyword>図</keyword>
        <keyword>地図</keyword>
        <class type="NDC" version="9">345</class>
        <system>
            <reg-date>20321209121610</reg-date>
            <lst-date>20321225132313</lst-date>
            <sys-id>3100004203</sys-id>
            <lib-id>6210006</lib-id>
            <lib-name>中央図書館</lib-name>
            <file-num>0</file-num>
        </system>
        <url>https://crd.ndl.go.jp/reference/detail?page=col_view&amp;id=3100004203</url>
        </collection>"#;
        let collection: Collection = from_str(collection).unwrap();
        assert_eq!(collection.col_name, "地図");
        assert_eq!(collection.pro_key, "チズ");
        assert_eq!(collection.reg_id, "0000-000");
        assert_eq!(collection.outline, "内容");
        assert_eq!(collection.origin, None);
        assert_eq!(collection.restriction.unwrap(), "要図書館カード");
        assert_eq!(collection.catalog.unwrap(), "蔵書検索にて一覧表示が可能");
        assert_eq!(collection.literature.unwrap(), "ホームページ");
        assert_eq!(collection.number.unwrap(), "75点");
        assert_eq!(collection.collection_continue.unwrap(), false);
        assert_eq!(collection.keyword.unwrap(), ["図", "地図"]);
        assert_eq!(
            collection.class.unwrap(),
            [Class {
                class_type: "NDC".to_string(),
                version: Some("9".to_string()),
                class: "345".to_string()
            }]
        );
        assert_eq!(collection.note, None);
        assert_eq!(collection.system.sys_id, "3100004203");
        assert_eq!(collection.system.lib_id, "6210006");
        assert_eq!(collection.system.lib_name, "中央図書館");
        assert_eq!(collection.system.file_num, 0);
        assert_eq!(
            collection.url,
            "https://crd.ndl.go.jp/reference/detail?page=col_view&id=3100004203"
        );
    }

    #[test]
    fn profile_test() {
        let profile = "<profile>
        <lib-type>61</lib-type>
        <lib-name>資料館図書室</lib-name>
        <abbr>資料館</abbr>
        <pro-key>シリョウカントショシツ</pro-key>
        <zip-code>000-0002</zip-code>
        <add-pref>東京都</add-pref>
        <add-city>東京市</add-city>
        <add-street>東京町1-1-11</add-street>
        <tel1>000-000-0000</tel1>
        <fax>111-111-1111</fax>
        <e-mail>lib@example.jp</e-mail>
        <lib-url>https://www.example.jp/</lib-url>
        <open-info>休室日：蔵書点検期間。</open-info>
        <restriction>https://www.example.jp/services/library/</restriction>
        <feature>特徴</feature>
        <notes>利用者登録が必要です。</notes>
        <access>https://www.example.jp/information/access/</access>
        <isil>JP-4001495</isil>
        <system>
            <reg-date>20330221101300</reg-date>
            <lst-date>20330221145857</lst-date>
            <lib-id>6100012</lib-id>
            <lib-name>資料館図書室</lib-name>
            <file-num>0</file-num>
        </system>
        <url>https://crd.ndl.go.jp/reference/detail?page=pro_view&amp;id=6100012</url>
        </profile>";
        let profile: Profile = from_str(profile).unwrap();
        assert_eq!(profile.lib_type, "61");
        assert_eq!(profile.lib_name, "資料館図書室");
        assert_eq!(profile.abbr, "資料館");
        assert_eq!(profile.pro_key, "シリョウカントショシツ");
        assert_eq!(profile.zip_code, "000-0002");
        assert_eq!(profile.add_pref, "東京都");
        assert_eq!(profile.add_city, "東京市");
        assert_eq!(profile.add_street, "東京町1-1-11");
        assert_eq!(profile.tel1, "000-000-0000");
        assert_eq!(profile.tel1_note, None);
        assert_eq!(profile.fax.unwrap(), "111-111-1111");
        assert_eq!(profile.e_mail.unwrap(), "lib@example.jp");
        assert_eq!(profile.lib_url.unwrap(), "https://www.example.jp/");
        assert_eq!(profile.open_info.unwrap(), "休室日：蔵書点検期間。");
        assert_eq!(
            profile.restriction.unwrap(),
            "https://www.example.jp/services/library/"
        );
        assert_eq!(profile.feature.unwrap(), "特徴");
        assert_eq!(profile.notes.unwrap(), "利用者登録が必要です。");
        assert_eq!(
            profile.access.unwrap(),
            "https://www.example.jp/information/access/"
        );
        assert_eq!(profile.isil.unwrap(), "JP-4001495");
        assert_eq!(
            profile.system.reg_date,
            NaiveDate::from_ymd_opt(2033, 2, 21)
                .unwrap()
                .and_hms_opt(10, 13, 00)
                .unwrap()
        );
        assert_eq!(
            profile.system.lst_date,
            NaiveDate::from_ymd_opt(2033, 2, 21)
                .unwrap()
                .and_hms_opt(14, 58, 57)
                .unwrap()
        );
        assert_eq!(profile.system.lib_id, "6100012");
        assert_eq!(profile.system.lib_name, "資料館図書室");
        assert_eq!(profile.system.file_num, 0);
        assert_eq!(
            profile.url,
            "https://crd.ndl.go.jp/reference/detail?page=pro_view&id=6100012"
        )
    }

    #[test]
    fn class_test() {
        let class = r#"<class type="NDC">913</class>"#;
        let class: Class = from_str(class).unwrap();
        assert_eq!(class.class_type, "NDC");
        assert_eq!(class.version, None);
        assert_eq!(class.class, "913");
    }

    #[test]
    fn bibl_test() {
        let bibl = "<bibl>
        <bibl-desc>書誌的事項</bibl-desc>
        <bibl-isbn>9794840121361</bibl-isbn>
        <bibl-note>当館請求記号</bibl-note>
        </bibl>";
        let bibl: Bibl = from_str(bibl).unwrap();
        assert_eq!(bibl.bibl_desc.unwrap(), "書誌的事項");
        assert_eq!(bibl.bibl_isbn.unwrap(), "9794840121361");
        assert_eq!(bibl.bibl_note.unwrap(), "当館請求記号".to_string());
    }

    #[test]
    fn system_test() {
        let system = "<system>
        <reg-date>20330211125936</reg-date>
        <lst-date>20330221193745</lst-date>
        <sys-id>1100328823</sys-id>
        <lib-id>6110044</lib-id>
        <lib-name>図書館</lib-name>
        <file-num>0</file-num>
        </system>";
        let system: System = from_str(system).unwrap();
        assert_eq!(
            system.reg_date,
            NaiveDate::from_ymd_opt(2033, 2, 11)
                .unwrap()
                .and_hms_opt(12, 59, 36)
                .unwrap()
        );
        assert_eq!(
            system.lst_date,
            NaiveDate::from_ymd_opt(2033, 2, 21)
                .unwrap()
                .and_hms_opt(19, 37, 45)
                .unwrap()
        );
        assert_eq!(system.sys_id, "1100328823");
        assert_eq!(system.lib_id, "6110044");
        assert_eq!(system.lib_name, "図書館");
        assert_eq!(system.file_num, 0);
    }
}
