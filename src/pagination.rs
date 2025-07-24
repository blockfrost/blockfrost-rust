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
