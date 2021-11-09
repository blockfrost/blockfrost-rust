//! Asynchronous and infinite lister.
//!
//! See [`Lister`].

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use crate::{
    request::send_get_request,
    stream::{FuturesOrdered, Stream},
    url::Url,
    *,
};

type ListerFutureInner<'api, T> = dyn Future<Output = crate::Result<T>> + Send + 'api;
type ListerFuture<'api, T> = Pin<Box<ListerFutureInner<'api, T>>>;

/// Infinite stream for paginated results.
///
/// Implements [`Stream`] from [`futures`], it's highly recommended to be used with
/// [`blockfrost::stream`](crate::stream).
///
/// To fully use the potential of this crate
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
            let BlockFrostApi { settings, client } = &self.api;
            let page = Some(self.current_page);

            let Url(url) = Url::from_endpoint_with_page(settings, &self.endpoint, page);
            let future = send_get_request(client, url);
            self.inner.push(Box::pin(future));

            // Increment page for next futures
            self.current_page += 1;
        }

        // Next item
        Pin::new(&mut self.inner).poll_next(context)
    }
}
