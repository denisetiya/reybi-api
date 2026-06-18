use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct PaginationQuery {
    pub cursor: Option<String>,
    pub limit: Option<i64>,
}

impl PaginationQuery {
    pub fn take(&self) -> i64 {
        self.limit.unwrap_or(25).min(100).max(1)
    }
}

pub fn paginate<T>(items: &[T], limit: i64) -> (Vec<T>, Option<String>, bool)
where
    T: HasCursor,
    T: Clone,
{
    let has_more = items.len() as i64 > limit;
    let data: Vec<T> = if has_more {
        items[..items.len() - 1].to_vec()
    } else {
        items.to_vec()
    };
    let cursor = if has_more {
        data.last().map(|item| item.cursor_value())
    } else {
        None
    };
    (data, cursor, has_more)
}

pub trait HasCursor {
    fn cursor_value(&self) -> String;
}
