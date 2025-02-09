//! Deletes one or more videos. Videos are past broadcasts, Highlights, or uploads.
//! [`delete-videos`](https://dev.twitch.tv/docs/api/reference#delete-videos)
//!
//! # Accessing the endpoint
//!
//! ## Request: [DeleteVideosRequest]
//!
//! To use this endpoint, construct a [`DeleteVideosRequest`] with the [`DeleteVideosRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::videos::delete_videos;
//! let request = delete_videos::DeleteVideosRequest::builder()
//!     .id(vec!["1234".to_string()])
//!     .build();
//! ```
//!
//! ## Response: [DeleteVideo]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, videos::delete_videos};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = delete_videos::DeleteVideosRequest::builder()
//!     .id(vec!["1234".to_string()])
//!     .build();
//! let response: delete_videos::DeleteVideo = client.req_delete(request, &token).await?;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestDelete::create_request)
//! and parse the [`http::Response`] with [`request.parse_response(&request.get_uri()?)`](helix::RequestDelete::parse_response())

use super::*;

// FIXME: One of id, user_id or game_id needs to be specified. typed_builder should have enums. id can not be used with other params
/// Query Parameters for [Get Videos](super::delete_videos)
///
/// [`delete-videos`](https://dev.twitch.tv/docs/api/reference#delete-videos)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct DeleteVideosRequest {
    /// ID of the video(s) to be deleted. Limit: 5.
    #[builder(default)]
    pub id: Vec<types::VideoId>,
}
// FIXME: Should return VideoIds
/// Return Values for [Get Videos](super::delete_videos)
///
/// [`delete-videos`](https://dev.twitch.tv/docs/api/reference#delete-videos)
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub enum DeleteVideo {
    /// Video(s) deleted.
    Success,
    /// Request was invalid.
    InvalidRequest,
    /// Authorization failed.
    AuthFailed,
}

impl std::convert::TryFrom<http::StatusCode> for DeleteVideo {
    type Error = std::borrow::Cow<'static, str>;

    fn try_from(s: http::StatusCode) -> Result<Self, Self::Error> {
        match s {
            http::StatusCode::OK => Ok(DeleteVideo::Success),
            http::StatusCode::BAD_REQUEST => Ok(DeleteVideo::InvalidRequest),
            http::StatusCode::UNAUTHORIZED => Ok(DeleteVideo::AuthFailed),
            other => Err(other.canonical_reason().unwrap_or("").into()),
        }
    }
}

impl helix::Request for DeleteVideosRequest {
    type Response = DeleteVideo;

    const PATH: &'static str = "videos";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelManageVideos];
}

impl helix::RequestDelete for DeleteVideosRequest {}

#[test]
fn test_request() {
    use helix::*;
    let req = DeleteVideosRequest::builder()
        .id(vec!["234482848".to_string()])
        .build();

    // From twitch docs
    let data = br#""#.to_vec();

    let http_response = http::Response::builder().status(200).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/videos?id=234482848"
    );

    dbg!(DeleteVideosRequest::parse_response(&uri, http_response).unwrap());
}
