use handle_errors::Error;
use std::collections::HashMap;

/// Pagination struct that is getting extracted
/// from query parameter
#[derive(Debug, Default)]
pub struct Pagination {
    /// The index of the last item that has to be returned
    pub limit: Option<u32>,
    /// The index of the first item that has to be returned
    pub offset: u32,
}

/// Extract query parameters from the `/questions` route
/// # Example query
/// GET request to this route can have a pagination attached
/// so we just return the questions we need
/// `/questions?start=1&end=5`
/// # Example usage
/// ```rust
/// let mut query = HashMap::new();
/// query.insert("limit".to_string(), "1".to_string());
/// query.insert("offset".to_string(), "10".to_string());
/// let p = types::pagination::extract_pagination(query).unwrap();
/// assert_eq!(p.limit, Some(10));
/// assert_eq!(p.offset, 10);
/// ```

pub fn extract_pagination(params: &HashMap<String, String>) -> Result<Pagination, Error> {
    // could be improved in future
    if params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            // Takes the "limit" parameter in the query
            // and tries to convert it into a number
            limit: Some(
                params
                    .get("limit")
                    .unwrap()
                    .parse::<u32>()
                    .map_err(Error::ParseError)?,
            ),
            // Takes the "offset" parameter in the query
            // and tries to convert it into a number
            offset: params
                .get("offset")
                .unwrap()
                .parse::<u32>()
                .map_err(Error::ParseError)?,
        });
    }
    Err(Error::MissingParameters)
}
