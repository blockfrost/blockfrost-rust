use crate::{DEFAULT_ORDER, DEFAULT_PAGINATION_PAGE_COUNT, DEFAULT_PAGINATION_PAGE_ITEMS_COUNT};

#[derive(Clone, Copy)]
pub struct Pagination {
    pub fetch_all: bool,
    pub count: usize,
    pub page: usize,
    pub order: Order,
}

impl Default for Pagination {
    fn default() -> Self {
        Pagination {
            fetch_all: false,
            count: DEFAULT_PAGINATION_PAGE_ITEMS_COUNT,
            page: DEFAULT_PAGINATION_PAGE_COUNT,
            order: DEFAULT_ORDER,
        }
    }
}

impl Pagination {
    pub fn new(order: Order, page: usize, count: usize) -> Self {
        Pagination {
            fetch_all: false,
            order,
            page,
            count,
        }
    }

    pub fn all() -> Self {
        Pagination {
            fetch_all: true,
            ..Default::default()
        }
    }

    /// Add an inclusive lower block cursor to this pagination.
    ///
    /// Transaction endpoints accept either a block number (`"8929261"`) or a block number and
    /// transaction index (`"8929261:10"`).
    pub fn with_from(self, from: impl Into<String>) -> CursorPagination {
        CursorPagination::from(self).with_from(from)
    }

    /// Add an inclusive upper block cursor to this pagination.
    ///
    /// Transaction endpoints accept either a block number (`"9999269"`) or a block number and
    /// transaction index (`"9999269:10"`).
    pub fn with_to(self, to: impl Into<String>) -> CursorPagination {
        CursorPagination::from(self).with_to(to)
    }

    /// Add inclusive lower and upper block cursors to this pagination.
    pub fn with_range(self, from: impl Into<String>, to: impl Into<String>) -> CursorPagination {
        CursorPagination::from(self).with_range(from, to)
    }

    pub fn order_to_string(&self) -> String {
        match self.order {
            Order::Asc => "asc".to_string(),
            Order::Desc => "desc".to_string(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Order {
    Asc,
    Desc,
}

/// Pagination with optional inclusive block cursors.
///
/// This is supported by the account, address, and asset transaction endpoints. Construct it from
/// [`Pagination`] with [`Pagination::with_from`], [`Pagination::with_to`], or
/// [`Pagination::with_range`]. A plain [`Pagination`] remains accepted by those endpoints.
#[derive(Clone)]
pub struct CursorPagination {
    pub pagination: Pagination,
    pub from: Option<String>,
    pub to: Option<String>,
}

impl CursorPagination {
    pub fn new(pagination: Pagination) -> Self {
        Self {
            pagination,
            from: None,
            to: None,
        }
    }

    pub fn with_from(mut self, from: impl Into<String>) -> Self {
        self.from = Some(from.into());
        self
    }

    pub fn with_to(mut self, to: impl Into<String>) -> Self {
        self.to = Some(to.into());
        self
    }

    pub fn with_range(mut self, from: impl Into<String>, to: impl Into<String>) -> Self {
        self.from = Some(from.into());
        self.to = Some(to.into());
        self
    }
}

impl From<Pagination> for CursorPagination {
    fn from(pagination: Pagination) -> Self {
        Self::new(pagination)
    }
}

impl Default for CursorPagination {
    fn default() -> Self {
        Self::new(Pagination::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pagination_builds_cursor_range() {
        let cursor = Pagination::default().with_range("8929261", "9999269:10");

        assert_eq!(cursor.from.as_deref(), Some("8929261"));
        assert_eq!(cursor.to.as_deref(), Some("9999269:10"));
        assert_eq!(cursor.pagination.page, DEFAULT_PAGINATION_PAGE_COUNT);
        assert_eq!(cursor.pagination.count, DEFAULT_PAGINATION_PAGE_ITEMS_COUNT);
    }
}
