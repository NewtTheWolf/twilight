use super::RequestReactionType;
use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use twilight_model::id::{ChannelId, MessageId, UserId};

/// User to delete the reaction of.
pub(crate) enum TargetUser {
    /// Delete a reaction of the current user.
    Current,
    /// Delete a reaction from a user by their ID.
    Id(UserId),
}

/// Delete one reaction by a user on a message.
#[must_use = "requests must be configured and executed"]
pub struct DeleteReaction<'a> {
    channel_id: ChannelId,
    emoji: &'a RequestReactionType<'a>,
    http: &'a Client,
    message_id: MessageId,
    target_user: TargetUser,
}

impl<'a> DeleteReaction<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: &'a RequestReactionType<'a>,
        target_user: TargetUser,
    ) -> Self {
        Self {
            channel_id,
            emoji,
            http,
            message_id,
            target_user,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for DeleteReaction<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let route = match self.target_user {
            TargetUser::Current => Route::DeleteReactionCurrentUser {
                channel_id: self.channel_id.get(),
                emoji: self.emoji,
                message_id: self.message_id.get(),
            },
            TargetUser::Id(user_id) => Route::DeleteReaction {
                channel_id: self.channel_id.get(),
                emoji: self.emoji,
                message_id: self.message_id.get(),
                user_id: user_id.get(),
            },
        };

        Ok(Request::from_route(&route))
    }
}
