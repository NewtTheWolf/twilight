use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use twilight_model::{channel::thread::ThreadsListing, id::ChannelId};

/// Returns archived private threads in the channel that the current user has
/// joined.
///
/// Threads are ordered by their ID in descending order.
#[must_use = "requests must be configured and executed"]
pub struct GetJoinedPrivateArchivedThreads<'a> {
    before: Option<ChannelId>,
    channel_id: ChannelId,
    http: &'a Client,
    limit: Option<u64>,
}

impl<'a> GetJoinedPrivateArchivedThreads<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self {
            before: None,
            channel_id,
            http,
            limit: None,
        }
    }

    /// Return threads before this ID.
    pub const fn before(mut self, before: ChannelId) -> Self {
        self.before = Some(before);

        self
    }

    /// Maximum number of threads to return.
    pub const fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ThreadsListing> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetJoinedPrivateArchivedThreads<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(
            &Route::GetJoinedPrivateArchivedThreads {
                before: self.before.map(ChannelId::get),
                channel_id: self.channel_id.get(),
                limit: self.limit,
            },
        ))
    }
}
