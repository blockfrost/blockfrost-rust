use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use crate::{
    stream::{FuturesOrdered, Stream},
    url::Url,
    *,
};

type ListerFutureInner<'api, T> = dyn Future<Output = crate::Result<T>> + Send + 'api;
type ListerFuture<'api, T> = Pin<Box<ListerFutureInner<'api, T>>>;

pub struct Lister<'api, T> {
    inner: FuturesOrdered<ListerFuture<'api, T>>,
    api: &'api BlockFrostApi,
    endpoint: String,
    current_page: u64,
}

impl<T> Lister<'_, T> {
    pub(crate) fn list_from_endpoint(api: &BlockFrostApi, endpoint: String) -> Lister<'_, T> {
        let inner = FuturesOrdered::<ListerFuture<T>>::new();
        let current_page = api.settings.query_parameters().page.unwrap_or(1);
        Lister { inner, endpoint, api, current_page }
    }
}

impl<'api, T: 'api + for<'de> serde::Deserialize<'de>> Stream for Lister<'api, T> {
    type Item = crate::Result<T>;

    fn poll_next(mut self: Pin<&mut Self>, context: &mut Context) -> Poll<Option<Self::Item>> {
        while self.inner.len() < 10 {
            // Making the next requests
            let settings = &self.api.settings;
            let endpoint = &self.endpoint;
            let page = Some(self.current_page);

            let Url(url) = Url::from_endpoint_with_page(settings, endpoint, page);
            let future = self.api.get_from_url(url);
            self.inner.push(Box::pin(future));

            // Increment page for next futures
            self.current_page += 1;
        }

        // Next item
        Pin::new(&mut self.inner).poll_next(context)
    }
}
