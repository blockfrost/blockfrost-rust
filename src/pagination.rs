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
            count: 100,
            page: 1,
            order: Order::Asc,
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
