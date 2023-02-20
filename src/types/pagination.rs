use handle_errors::Error;
use std::collections::HashMap;

/// Paginationの構造体であり、クエリパラメータから値を抽出する
#[derive(Debug)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}

/// "/questions" パスのクエリパラメータを抽出する
/// # サンプルクエリ
/// `/questions?start=1&end=10`
/// # サンプル
/// ```rust
/// use std::collections::HashMap;
///
/// let mut query = HashMap::new();
/// query.insert("start".to_string(), "1".to_string());
/// query.insert("end".to_string(), "10".to_string());
/// let p = pagination::extract_pagination(query).unwrap();
/// assert(p.start, 1);
/// assert(p.end, 10);
/// ```
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
        });
    }

    Err(Error::MissingParameters)
}