//! Get the usage data of a Deepgram Project.
//!
//! See the [Deepgram API Reference][api] for more info.
//!
//! [api]: https://developers.deepgram.com/api-reference/#usage

use crate::{send_and_translate_response, Deepgram};

pub mod get_fields_options;
pub mod get_usage_options;
pub mod list_requests_options;
pub mod response;

use response::{Fields, Request, Requests, UsageSummary};

/// Get the usage data of a Deepgram Project.
///
/// Constructed using [`Deepgram::usage`].
///
/// See the [Deepgram API Reference][api] for more info.
///
/// [api]: https://developers.deepgram.com/api-reference/#usage
#[derive(Debug, Clone)]
pub struct Usage<'a, K: AsRef<str>>(&'a Deepgram<K>);

impl<'a, K: AsRef<str>> Deepgram<K> {
    /// Construct a new [`Usage`] from a [`Deepgram`].
    pub fn usage(&'a self) -> Usage<'a, K> {
        self.into()
    }
}

impl<'a, K: AsRef<str>> From<&'a Deepgram<K>> for Usage<'a, K> {
    /// Construct a new [`Usage`] from a [`Deepgram`].
    fn from(deepgram: &'a Deepgram<K>) -> Self {
        Self(deepgram)
    }
}

impl<'a, K: AsRef<str>> Usage<'_, K> {
    /// Get all requests sent to the Deepgram API for the specified project.
    ///
    /// See the [Deepgram API Reference][api] for more info.
    ///
    /// [api]: https://developers.deepgram.com/api-reference/#usage-all
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::env;
    /// #
    /// # use deepgram::{
    /// #     usage::{get_fields_options, get_usage_options, list_requests_options},
    /// #     Deepgram, DeepgramError,
    /// # };
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), DeepgramError> {
    /// # let deepgram_api_key =
    /// #     env::var("DEEPGRAM_API_KEY").expect("DEEPGRAM_API_KEY environmental variable");
    /// #
    /// # let project_id =
    /// #     env::var("DEEPGRAM_PROJECT_ID").expect("DEEPGRAM_PROJECT_ID environmental variable");
    /// #
    /// let dg_client = Deepgram::new(&deepgram_api_key);
    ///
    /// let options = list_requests_options::Options::builder().build();
    /// let requests = dg_client
    ///     .usage()
    ///     .list_requests(&project_id, &options)
    ///     .await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_requests(
        &self,
        project_id: &str,
        options: &list_requests_options::Options<'a>,
    ) -> crate::Result<Requests> {
        let url = format!(
            "https://api.deepgram.com/v1/projects/{}/requests",
            project_id,
        );
        let request = self
            .0
            .client
            .get(url)
            .query(&list_requests_options::SerializableOptions::from(options));

        send_and_translate_response(request).await
    }

    /// Get the details of the specified request sent to the Deepgram API for the specified project.
    ///
    /// See the [Deepgram API Reference][api] for more info.
    ///
    /// [api]: https://developers.deepgram.com/api-reference/#usage-get
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::env;
    /// #
    /// # use deepgram::{
    /// #     usage::{get_fields_options, get_usage_options, list_requests_options},
    /// #     Deepgram, DeepgramError,
    /// # };
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), DeepgramError> {
    /// # let deepgram_api_key =
    /// #     env::var("DEEPGRAM_API_KEY").expect("DEEPGRAM_API_KEY environmental variable");
    /// #
    /// # let project_id =
    /// #     env::var("DEEPGRAM_PROJECT_ID").expect("DEEPGRAM_PROJECT_ID environmental variable");
    /// #
    /// # let request_id =
    /// #     env::var("DEEPGRAM_REQUEST_ID").expect("DEEPGRAM_REQUEST_ID environmental variable");
    /// #
    /// let dg_client = Deepgram::new(&deepgram_api_key);
    ///
    /// let request = dg_client
    ///     .usage()
    ///     .get_request(&project_id, &request_id)
    ///     .await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_request(&self, project_id: &str, request_id: &str) -> crate::Result<Request> {
        let url = format!(
            "https://api.deepgram.com/v1/projects/{}/requests/{}",
            project_id, request_id,
        );

        send_and_translate_response(self.0.client.get(url)).await
    }

    /// Get a summary of usage statistics.
    ///
    /// See the [Deepgram API Reference][api] for more info.
    ///
    /// [api]: https://developers.deepgram.com/api-reference/#usage-summary
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::env;
    /// #
    /// # use deepgram::{
    /// #     usage::{get_fields_options, get_usage_options, list_requests_options},
    /// #     Deepgram, DeepgramError,
    /// # };
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), DeepgramError> {
    /// # let deepgram_api_key =
    /// #     env::var("DEEPGRAM_API_KEY").expect("DEEPGRAM_API_KEY environmental variable");
    /// #
    /// # let project_id =
    /// #     env::var("DEEPGRAM_PROJECT_ID").expect("DEEPGRAM_PROJECT_ID environmental variable");
    /// #
    /// let dg_client = Deepgram::new(&deepgram_api_key);
    ///
    /// let options = get_usage_options::Options::builder().build();
    /// let summary = dg_client
    ///     .usage()
    ///     .get_usage(&project_id, &options)
    ///     .await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_usage(
        &self,
        project_id: &str,
        options: &get_usage_options::Options<'a>,
    ) -> crate::Result<UsageSummary> {
        let url = format!("https://api.deepgram.com/v1/projects/{}/usage", project_id);
        let request = self
            .0
            .client
            .get(url)
            .query(&get_usage_options::SerializableOptions::from(options));

        send_and_translate_response(request).await
    }

    /// Get the features, models, tags, languages, and processing method used for requests in the specified project.
    ///
    /// See the [Deepgram API Reference][api] for more info.
    ///
    /// [api]: https://developers.deepgram.com/api-reference/#usage-fields
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::env;
    /// #
    /// # use deepgram::{
    /// #     usage::{get_fields_options, get_usage_options, list_requests_options},
    /// #     Deepgram, DeepgramError,
    /// # };
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), DeepgramError> {
    /// # let deepgram_api_key =
    /// #     env::var("DEEPGRAM_API_KEY").expect("DEEPGRAM_API_KEY environmental variable");
    /// #
    /// # let project_id =
    /// #     env::var("DEEPGRAM_PROJECT_ID").expect("DEEPGRAM_PROJECT_ID environmental variable");
    /// #
    /// let dg_client = Deepgram::new(&deepgram_api_key);
    ///
    /// let options = get_fields_options::Options::builder().build();
    /// let summary = dg_client
    ///     .usage()
    ///     .get_fields(&project_id, &options)
    ///     .await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_fields(
        &self,
        project_id: &str,
        options: &get_fields_options::Options<'a>,
    ) -> crate::Result<Fields> {
        let url = format!(
            "https://api.deepgram.com/v1/projects/{}/usage/fields",
            project_id,
        );
        let request = self
            .0
            .client
            .get(url)
            .query(&get_fields_options::SerializableOptions::from(options));

        send_and_translate_response(request).await
    }
}
