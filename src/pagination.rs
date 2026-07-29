use crate::{DEFAULT_ORDER, DEFAULT_PAGINATION_PAGE_COUNT, DEFAULT_PAGINATION_PAGE_ITEMS_COUNT};
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Copy)]
pub struct Pagination {
    pub fetch_all: bool,
    pub count: usize,
    pub page: usize,
    pub order: Order,
    pub from: Option<BlockCursor>,
    pub to: Option<BlockCursor>,
}

impl Default for Pagination {
    fn default() -> Self {
        Pagination {
            fetch_all: false,
            count: DEFAULT_PAGINATION_PAGE_ITEMS_COUNT,
            page: DEFAULT_PAGINATION_PAGE_COUNT,
            order: DEFAULT_ORDER,
            from: None,
            to: None,
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
            from: None,
            to: None,
        }
    }

    pub fn all() -> Self {
        Pagination {
            fetch_all: true,
            ..Default::default()
        }
    }

    pub fn with_from(mut self, from: impl Into<BlockCursor>) -> Self {
        self.from = Some(from.into());
        self
    }

    pub fn with_to(mut self, to: impl Into<BlockCursor>) -> Self {
        self.to = Some(to.into());
        self
    }

    pub fn with_range(self, from: impl Into<BlockCursor>, to: impl Into<BlockCursor>) -> Self {
        self.with_from(from).with_to(to)
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockCursor {
    pub block_height: u64,
    pub tx_index: Option<u32>,
}

impl BlockCursor {
    pub fn block(block_height: u64) -> Self {
        Self {
            block_height,
            tx_index: None,
        }
    }

    pub fn tx(block_height: u64, tx_index: u32) -> Self {
        Self {
            block_height,
            tx_index: Some(tx_index),
        }
    }
}

impl fmt::Display for BlockCursor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.tx_index {
            Some(tx_index) => write!(f, "{}:{}", self.block_height, tx_index),
            None => write!(f, "{}", self.block_height),
        }
    }
}

impl From<u64> for BlockCursor {
    fn from(block_height: u64) -> Self {
        Self::block(block_height)
    }
}

impl From<(u64, u32)> for BlockCursor {
    fn from((block_height, tx_index): (u64, u32)) -> Self {
        Self::tx(block_height, tx_index)
    }
}

impl FromStr for BlockCursor {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(':') {
            Some((block_height, tx_index)) => {
                Ok(Self::tx(block_height.parse()?, tx_index.parse()?))
            }
            None => Ok(Self::block(s.parse()?)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pagination_builds_cursor_range() {
        let pagination = Pagination::default().with_range(8929261, (9999269, 10));

        assert_eq!(pagination.from, Some(BlockCursor::block(8929261)));
        assert_eq!(pagination.to, Some(BlockCursor::tx(9999269, 10)));
        assert_eq!(pagination.page, DEFAULT_PAGINATION_PAGE_COUNT);
        assert_eq!(pagination.count, DEFAULT_PAGINATION_PAGE_ITEMS_COUNT);
    }

    #[test]
    fn pagination_stays_copy() {
        let pagination = Pagination::default().with_from(8929261);
        let copied = pagination;

        assert_eq!(pagination.from, copied.from);
    }

    #[test]
    fn block_cursor_display() {
        assert_eq!(BlockCursor::block(8929261).to_string(), "8929261");
        assert_eq!(BlockCursor::tx(9999269, 10).to_string(), "9999269:10");
    }

    #[test]
    fn block_cursor_from_str() {
        assert_eq!("8929261".parse(), Ok(BlockCursor::block(8929261)));
        assert_eq!("9999269:10".parse(), Ok(BlockCursor::tx(9999269, 10)));
        assert!("nope".parse::<BlockCursor>().is_err());
    }
}
