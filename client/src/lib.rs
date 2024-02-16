#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
pub mod types {
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use std::convert::TryFrom;
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AddTicketToBasketRequest {
        ///Duration in days
        pub duration: i32,
        pub ticket_type_id: String,
    }

    impl From<&AddTicketToBasketRequest> for AddTicketToBasketRequest {
        fn from(value: &AddTicketToBasketRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AddUserInfoRequest {
        pub address: String,
        pub email: String,
        pub name: String,
    }

    impl From<&AddUserInfoRequest> for AddUserInfoRequest {
        fn from(value: &AddUserInfoRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum ApiError {
        DbExecutionError(String),
        FailedPrecondition(String),
        NotFound(String),
        Unknown,
    }

    impl From<&ApiError> for ApiError {
        fn from(value: &ApiError) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Order {
        pub duration: i32,
        pub id: uuid::Uuid,
        pub price: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub purchased_at: Option<chrono::DateTime<chrono::offset::Utc>>,
        pub reserved_until: chrono::DateTime<chrono::offset::Utc>,
        pub ticket_type_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub user_id: Option<String>,
    }

    impl From<&Order> for Order {
        fn from(value: &Order) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TicketType {
        pub display: String,
        pub id: String,
        pub sold_out: bool,
    }

    impl From<&TicketType> for TicketType {
        fn from(value: &TicketType) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct User {
        pub address: String,
        pub email: String,
        pub id: uuid::Uuid,
        pub name: String,
        pub order_id: uuid::Uuid,
    }

    impl From<&User> for User {
        fn from(value: &User) -> Self {
            value.clone()
        }
    }
}

#[derive(Clone, Debug)]
///Client for festival-tickets-actix
///
///
///
///Version: 0.1.0
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
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
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

    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }

    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI
    /// document and may be in any format the API selects.
    pub fn api_version(&self) -> &'static str {
        "0.1.0"
    }
}

impl Client {
    ///Retrieve an order by ID
    ///
    ///Retrieve an order by ID
    ///
    ///Sends a `GET` request to `/orders/{order_id}`
    pub async fn get_order<'a>(
        &'a self,
        order_id: &'a uuid::Uuid,
    ) -> Result<ResponseValue<types::Order>, Error<types::ApiError>> {
        let url = format!(
            "{}/orders/{}",
            self.baseurl,
            encode_path(&order_id.to_string()),
        );
        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Add user info to order
    ///
    ///Add user info to order
    ///
    ///Sends a `POST` request to `/orders/{order_id}/add-user-info`
    ///
    ///Arguments:
    /// - `order_id`
    /// - `body`:
    pub async fn add_user_info<'a>(
        &'a self,
        order_id: &'a uuid::Uuid,
        body: &'a types::AddUserInfoRequest,
    ) -> Result<ResponseValue<types::Order>, Error<()>> {
        let url = format!(
            "{}/orders/{}/add-user-info",
            self.baseurl,
            encode_path(&order_id.to_string()),
        );
        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Purchase an order. Note: User info must be attached to order first
    ///
    ///Purchase an order. Note: User info must be attached to order first
    ///
    ///Sends a `POST` request to `/orders/{order_id}/purchase`
    pub async fn purchase_order<'a>(
        &'a self,
        order_id: &'a uuid::Uuid,
    ) -> Result<ResponseValue<types::Order>, Error<types::ApiError>> {
        let url = format!(
            "{}/orders/{}/purchase",
            self.baseurl,
            encode_path(&order_id.to_string()),
        );
        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Add Ticket of type and duration in days to basket
    ///
    ///Add Ticket of type and duration in days to basket
    ///
    ///Sends a `POST` request to `/tickets/add-to-basket`
    ///
    ///Arguments:
    /// - `body`:
    pub async fn add_ticket_to_basket<'a>(
        &'a self,
        body: &'a types::AddTicketToBasketRequest,
    ) -> Result<ResponseValue<types::Order>, Error<types::ApiError>> {
        let url = format!("{}/tickets/add-to-basket", self.baseurl,);
        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List possible duration (days) selection for given ticket type
    ///
    ///List possible duration (days) selection for given ticket type
    ///
    ///Sends a `GET` request to `/tickets/durations/{ticket_type_id}`
    pub async fn get_ticket_durations<'a>(
        &'a self,
        ticket_type_id: &'a str,
    ) -> Result<ResponseValue<Vec<i32>>, Error<()>> {
        let url = format!(
            "{}/tickets/durations/{}",
            self.baseurl,
            encode_path(&ticket_type_id.to_string()),
        );
        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List possible ticket types
    ///
    ///List possible ticket types
    ///
    ///Sends a `GET` request to `/tickets/types`
    pub async fn get_ticket_types<'a>(
        &'a self,
    ) -> Result<ResponseValue<Vec<types::TicketType>>, Error<()>> {
        let url = format!("{}/tickets/types", self.baseurl,);
        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Retrieve a user by ID
    ///
    ///Retrieve a user by ID
    ///
    ///Sends a `GET` request to `/users/{user_id}`
    pub async fn get_user<'a>(
        &'a self,
        user_id: &'a uuid::Uuid,
    ) -> Result<ResponseValue<types::User>, Error<types::ApiError>> {
        let url = format!(
            "{}/users/{}",
            self.baseurl,
            encode_path(&user_id.to_string()),
        );
        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

pub mod prelude {
    pub use super::Client;
}
