use app::ddb::Cursor;
use app::errors::AppError;
use async_graphql::connection::{Connection, CursorType, Edge, EmptyFields};
use async_graphql::OutputType;

#[derive(Clone, Debug)]
pub struct Pagination {
    pub limit: i32,
    pub forward: bool,
    pub cursor: Option<Cursor>,
}

impl Pagination {
    pub fn calc(
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
        default_limit: i32,
    ) -> Result<Self, AppError> {
        let (limit, forward, cursor) = match (after, before, first, last) {
            (Some(_), Some(_), ..) => Err(AppError::bad_request())?,
            (Some(_), _, _, Some(_)) => Err(AppError::bad_request())?,
            (_, Some(_), Some(_), _) => Err(AppError::bad_request())?,
            (_, _, Some(_), Some(_)) => Err(AppError::bad_request())?,
            (Some(after), None, first, None) => (
                first.unwrap_or(default_limit),
                true,
                Some(Cursor::from(after)),
            ),
            (after, None, Some(first), None) => (first, true, after.map(Cursor::from)),
            (None, Some(before), None, last) => (
                last.unwrap_or(default_limit),
                false,
                Some(Cursor::from(before)),
            ),
            (None, before, None, Some(last)) => (last, false, before.map(Cursor::from)),
            _ => (default_limit, false, None),
        };
        Ok(Self {
            limit: limit + 1,
            forward,
            cursor,
        })
    }

    pub fn connection<C: CursorType + Send + Sync, N: OutputType>(
        &self,
        edges: Vec<Edge<C, N, EmptyFields>>,
    ) -> Connection<C, N> {
        let (has_prev, has_next) = self.guess_has_prev_next(edges.len());
        let mut con = Connection::new(has_prev, has_next);
        let mut edges = edges
            .into_iter()
            .take(self.limit as usize)
            .collect::<Vec<_>>();

        if edges.len() >= self.limit as usize {
            edges.pop();
        }

        con.edges.append(&mut edges);
        con
    }

    pub fn guess_has_prev_next(&self, fetched_len: usize) -> (bool, bool) {
        let cursor_specified = self.cursor.is_some();
        let leach_limit = self.limit <= fetched_len as i32;

        if self.forward {
            (leach_limit, cursor_specified)
        } else {
            (cursor_specified, leach_limit)
        }
    }
}
