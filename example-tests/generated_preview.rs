//! Auto-generated client and tests for the Simple API

// Client code
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
#[allow(unused_imports)]
use progenitor_client::{encode_path, ClientHooks, OperationInfo, RequestBuilderExt};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a `TryFrom` or `FromStr` implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter<'_>,
            ) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }
        impl ::std::fmt::Debug for ConversionError {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter<'_>,
            ) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }
        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }
        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }
    ///`CreateUserRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default, PartialEq)]
    pub struct CreateUserRequest {
        pub name: ::std::string::String,
    }
    impl ::std::convert::From<&CreateUserRequest> for CreateUserRequest {
        fn from(value: &CreateUserRequest) -> Self {
            value.clone()
        }
    }
    ///`Error`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default, PartialEq)]
    pub struct Error {
        pub message: ::std::string::String,
    }
    impl ::std::convert::From<&Error> for Error {
        fn from(value: &Error) -> Self {
            value.clone()
        }
    }
    ///`User`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default, PartialEq)]
    pub struct User {
        pub id: ::std::string::String,
        pub name: ::std::string::String,
    }
    impl ::std::convert::From<&User> for User {
        fn from(value: &User) -> Self {
            value.clone()
        }
    }
    ///Error enum for the `list_users` operation
    #[derive(Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
    pub enum ListUsersError {
        #[doc = concat!("Error response for status code ", "500")]
        Status500(Error),
        /// Error response for an unknown status code
        UnknownValue(serde_json::Value),
    }
    impl std::str::FromStr for ListUsersError {
        type Err = std::string::String;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (status_code, value) = match s.split_once(':') {
                Some((status_code, value)) => (status_code, value),
                None => return Err("Unable to split status code and value".to_string()),
            };
            let status_code: u16 = match status_code.parse() {
                Ok(code) => code,
                Err(e) => return Err(format!("Unable to parse status code: {}", e)),
            };
            match status_code {
                500u16 => {
                    match serde_json::from_str(value) {
                        Ok(parsed_value) => Ok(Self::Status500(parsed_value)),
                        Err(_) => {
                            Err("Unable to parse error response as JSON".to_string())
                        }
                    }
                }
                _ => {
                    match serde_json::from_str(value) {
                        Ok(json_value) => Ok(Self::UnknownValue(json_value)),
                        Err(_) => Err("Unable to parse as JSON".to_string()),
                    }
                }
            }
        }
    }
    ///Error enum for the `create_user` operation
    #[derive(Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
    pub enum CreateUserError {
        #[doc = concat!("Error response for status code ", "400")]
        Status400(Error),
        /// Error response for an unknown status code
        UnknownValue(serde_json::Value),
    }
    impl std::str::FromStr for CreateUserError {
        type Err = std::string::String;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (status_code, value) = match s.split_once(':') {
                Some((status_code, value)) => (status_code, value),
                None => return Err("Unable to split status code and value".to_string()),
            };
            let status_code: u16 = match status_code.parse() {
                Ok(code) => code,
                Err(e) => return Err(format!("Unable to parse status code: {}", e)),
            };
            match status_code {
                400u16 => {
                    match serde_json::from_str(value) {
                        Ok(parsed_value) => Ok(Self::Status400(parsed_value)),
                        Err(_) => {
                            Err("Unable to parse error response as JSON".to_string())
                        }
                    }
                }
                _ => {
                    match serde_json::from_str(value) {
                        Ok(json_value) => Ok(Self::UnknownValue(json_value)),
                        Err(_) => Err("Unable to parse as JSON".to_string()),
                    }
                }
            }
        }
    }
}
#[derive(Clone, Debug)]
/**Client for Simple API

Version: 1.0.0*/
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}
impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new().connect_timeout(dur).timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(
            baseurl,
            client.build().expect("Failed to build HTTP client"),
        )
    }
    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
}
impl ClientInfo<()> for Client {
    fn api_version() -> &'static str {
        "1.0.0"
    }
    fn baseurl(&self) -> &str {
        self.baseurl.as_str()
    }
    fn client(&self) -> &reqwest::Client {
        &self.client
    }
    fn inner(&self) -> &() {
        &()
    }
}
impl ClientHooks<()> for &Client {}
#[allow(clippy::all)]
#[allow(elided_named_lifetimes)]
impl Client {
    /**List users

Sends a 'GET' request to '/users'

*/
    #[allow(unused_variables)]
    #[allow(irrefutable_let_patterns)]
    pub async fn list_users<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<::std::vec::Vec<types::User>>,
        Error<types::ListUsersError>,
    > {
        let url = format!("{}/users", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        #[allow(unused_variables)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_users",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            500u16 => {
                Err(
                    Error::ErrorResponse(
                        ResponseValue::<
                            types::ListUsersError,
                        >::from_response::<types::ListUsersError>(response)
                            .await?,
                    ),
                )
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Create user

Sends a 'POST' request to '/users'

*/
    #[allow(unused_variables)]
    #[allow(irrefutable_let_patterns)]
    pub async fn create_user<'a>(
        &'a self,
        body: &'a types::CreateUserRequest,
    ) -> Result<ResponseValue<types::User>, Error<types::CreateUserError>> {
        let url = format!("{}/users", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        #[allow(unused_mut)]
        #[allow(unused_variables)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_user",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(
                    Error::ErrorResponse(
                        ResponseValue::<
                            types::CreateUserError,
                        >::from_response::<types::CreateUserError>(response)
                            .await?,
                    ),
                )
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}
/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}


// HTTPMock support
pub mod httpmock {
pub mod operations {
    //! [`When`](::httpmock::When) and [`Then`](::httpmock::Then)
    //! wrappers for each operation. Each can be converted to
    //! its inner type with a call to `into_inner()`. This can
    //! be used to explicitly deviate from permitted values.
    use crate::*;
    pub struct ListUsersWhen(::httpmock::When);
    impl ListUsersWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(
                        regex::Regex::new("^/users$")
                            .expect("Invalid path regex pattern"),
                    ),
            )
        }
        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }
    pub struct ListUsersThen(::httpmock::Then);
    impl ListUsersThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }
        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }
        pub fn ok(self, value: &::std::vec::Vec<types::User>) -> Self {
            Self(
                self
                    .0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn internal_server_error(self, value: &types::Error) -> Self {
            Self(
                self
                    .0
                    .status(500u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
    pub struct CreateUserWhen(::httpmock::When);
    impl CreateUserWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(
                        regex::Regex::new("^/users$")
                            .expect("Invalid path regex pattern"),
                    ),
            )
        }
        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
        pub fn body(self, value: &types::CreateUserRequest) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }
    pub struct CreateUserThen(::httpmock::Then);
    impl CreateUserThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }
        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }
        pub fn created(self, value: &types::User) -> Self {
            Self(
                self
                    .0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn bad_request(self, value: &types::Error) -> Self {
            Self(
                self
                    .0
                    .status(400u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
}
/// An extension trait for [`MockServer`](::httpmock::MockServer) that
/// adds a method for each operation. These are the equivalent of
/// type-checked [`mock()`](::httpmock::MockServer::mock) calls.
pub trait MockServerExt {
    fn list_users<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ListUsersWhen, operations::ListUsersThen);
    fn create_user<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::CreateUserWhen, operations::CreateUserThen);
}
impl MockServerExt for ::httpmock::MockServer {
    fn list_users<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ListUsersWhen, operations::ListUsersThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ListUsersWhen::new(when),
                operations::ListUsersThen::new(then),
            )
        })
    }
    fn create_user<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::CreateUserWhen, operations::CreateUserThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::CreateUserWhen::new(when),
                operations::CreateUserThen::new(then),
            )
        })
    }
}

}

// Generated tests
#[cfg(test)]
#[cfg(test)]
pub mod generated_tests {
    use super::*;
    use crate::httpmock::{MockServerExt, operations};
    use ::httpmock::prelude::*;
    use tokio;
    #[tokio::test]
    ///Test successful List users with 200 response
    async fn test_list_users_success_200() {
        let server = MockServer::start();
        let client = Client::new(&server.base_url());
        let _mock = server
            .list_users(|when, then| {
                when;
                then.ok(&Vec::<types::User>::default());
            });
        let result = client.list_users().await;
        assert!(result.is_ok());
        let response = result.unwrap();
    }
    #[tokio::test]
    ///Test error case for List users with 500 response
    async fn test_list_users_error_500() {
        let server = MockServer::start();
        let client = Client::new(&server.base_url());
        let _mock = server
            .list_users(|when, then| {
                when;
                then.internal_server_error(&types::Error::default());
            });
        let result = client.list_users().await;
        assert!(result.is_err());
    }
    #[tokio::test]
    ///Test successful Create user with 201 response
    async fn test_create_user_success_201() {
        let server = MockServer::start();
        let client = Client::new(&server.base_url());
        let _mock = server
            .create_user(|when, then| {
                when.body(&types::CreateUserRequest::default());
                then.created(&types::User::default());
            });
        let result = client.create_user(&types::CreateUserRequest::default()).await;
        assert!(result.is_ok());
        let response = result.unwrap();
    }
    #[tokio::test]
    ///Test error case for Create user with 400 response
    async fn test_create_user_error_400() {
        let server = MockServer::start();
        let client = Client::new(&server.base_url());
        let _mock = server
            .create_user(|when, then| {
                when.body(&types::CreateUserRequest::default());
                then.bad_request(&types::Error::default());
            });
        let result = client.create_user(&types::CreateUserRequest::default()).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    ///Test create_user with parameter body
    async fn test_create_user_param_body() {
        let server = MockServer::start();
        let client = Client::new(&server.base_url());
    }
}

