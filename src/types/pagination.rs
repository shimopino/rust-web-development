use handle_errors::Error;
use std::collections::HashMap;

/// Paginationの構造体であり、クエリパラメータから値を抽出する
#[derive(Default, Debug, PartialEq)]
pub struct Pagination {
    /// 最後に返さなければならない項目のインデックス
    pub limit: Option<u32>,
    /// 返されなければならない最初の項目のインデックス
    pub offset: u32,
}

/// "/questions" パスのクエリパラメータを抽出する
/// # サンプルクエリ
/// `/questions?limit=1&offset=10`
/// # サンプル
/// ```rust
/// use std::collections::HashMap;
///
/// let mut query = HashMap::new();
/// query.insert("limit".to_string(), "1".to_string());
/// query.insert("offset".to_string(), "10".to_string());
/// let p = pagination::extract_pagination(query).unwrap();
/// assert(p.limit, Some(1));
/// assert(p.offset, 10);
/// ```
pub fn extract_pagination(
    params: HashMap<String, String>,
) -> Result<Pagination, Error> {
    if params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            limit: Some(
                params
                    .get("limit")
                    .unwrap()
                    .parse::<u32>()
                    .map_err(Error::ParseError)?,
            ),
            offset: params
                .get("offset")
                .unwrap()
                .parse::<u32>()
                .map_err(Error::ParseError)?,
        });
    }

    Err(Error::MissingParameters)
}

#[cfg(test)]
mod pagination_tests {
    use std::collections::HashMap;

    use super::{extract_pagination, Pagination};

    #[test]
    fn valid_pagination() {
        let mut params = HashMap::new();
        params.insert(String::from("limit"), String::from("1"));
        params.insert(String::from("offset"), String::from("10"));

        let pagination_result = extract_pagination(params);

        let expected = Pagination {
            limit: Some(1),
            offset: 10,
        };

        assert_eq!(pagination_result.unwrap(), expected);
    }

    #[test]
    fn missing_offset_parameter() {
        let mut params = HashMap::new();
        params.insert(String::from("limit"), String::from("1"));

        // std::fmt::Displayで実装されている文字列に変換する
        let pagination_result =
            format!("{}", extract_pagination(params).unwrap_err());

        let expected =
            format!("{}", handle_errors::Error::MissingParameters);

        println!("{}", pagination_result);
        println!("{}", expected);

        // エラーをそのまま比較検証する場合にはエラーに PartialEq が実装されている必要がある
        // しかし、sqlx::Error などの PartialEq を実装していまいものもあるので、Enumのエラーで
        // 比較検証する場合には、実装を追加しないといけない
        // そこでエラーを文字列に変換した後で比較検証をしてしまえばよい
        assert_eq!(pagination_result, expected)
    }

    #[test]
    fn missing_limit_parameter() {
        let mut params = HashMap::new();
        params.insert(String::from("offset"), String::from("10"));

        let pagination_result =
            format!("{}", extract_pagination(params).unwrap_err());

        let expected =
            format!("{}", handle_errors::Error::MissingParameters);

        assert_eq!(pagination_result, expected);
    }

    #[test]
    fn wrong_limit_parse() {
        let mut params = HashMap::new();
        params.insert(String::from("limit"), String::from("NOT_A_NUMBER"));
        params.insert(String::from("offset"), String::from("1"));

        let pagination_result =
            format!("{}", extract_pagination(params).unwrap_err());

        let expected = String::from(
            "Cannot parse parameter: invalid digit found in string",
        );

        assert_eq!(pagination_result, expected);
    }

    #[test]
    fn wrong_offset_parse() {
        let mut params = HashMap::new();
        params.insert(String::from("limit"), String::from("10"));
        params
            .insert(String::from("offset"), String::from("NOT_A_NUMBER"));

        let pagination_result =
            format!("{}", extract_pagination(params).unwrap_err());

        let expected = String::from(
            "Cannot parse parameter: invalid digit found in string",
        );

        assert_eq!(pagination_result, expected);
    }
}
