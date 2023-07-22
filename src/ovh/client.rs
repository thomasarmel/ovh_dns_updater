//! Inspired from https://github.com/MicroJoe/rust-ovh

use reqwest::{header::HeaderMap, Response, StatusCode};
use serde::Serialize;
use std::{
    convert::TryInto,
    time::{SystemTime, UNIX_EPOCH},
};

static ENDPOINTS: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "ovh-eu" => "https://eu.api.ovh.com/1.0",
    "ovh-us" => "https://api.us.ovhcloud.com/1.0",
    "ovh-ca" => "https://ca.api.ovh.com/1.0",
    "kimsufi-eu" => "https://eu.api.kimsufi.com/1.0",
    "kimsufi-ca" => "https://ca.api.kimsufi.com/1.0",
    "soyoustart-eu" => "https://eu.api.soyoustart.com/1.0",
    "soyoustart-ca" => "https://ca.api.soyoustart.com/1.0",
};

// Private helpers

fn insert_sensitive_header(headers: &mut HeaderMap, header_name: &'static str, value: &str) {
    let mut header_value = reqwest::header::HeaderValue::from_str(value).unwrap();
    header_value.set_sensitive(true);
    headers.insert(header_name, header_value);
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

// pub(crate)lic API

pub(crate) struct OvhClient {
    endpoint: &'static str,
    application_key: String,
    application_secret: String,
    consumer_key: String,
    client: reqwest::Client,
}

#[allow(dead_code)]
impl OvhClient {
    /// Creates a new client from scratch.
    pub(crate) fn new(
        endpoint: &str,
        application_key: &str,
        application_secret: &str,
        consumer_key: &str,
    ) -> Option<OvhClient> {
        let endpoint = ENDPOINTS.get(endpoint)?;
        let application_key = application_key.into();
        let application_secret = application_secret.into();
        let consumer_key = consumer_key.into();

        let client = reqwest::Client::new();

        Some(OvhClient {
            endpoint,
            application_key,
            application_secret,
            consumer_key,
            client,
        })
    }

    fn signature(&self, url: &str, timestamp: &str, method: &str, body: &str) -> String {
        let values = [
            &self.application_secret,
            &self.consumer_key,
            method,
            url,
            body,
            timestamp,
        ];
        let sha = sha1::Sha1::from(values.join("+")).hexdigest();
        format!("$1${}", sha)
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", &self.endpoint, path)
    }

    /// Retrieves the time delta between the local machine and the API server.
    ///
    /// This method will perform a request to the API server to get its
    /// local time, and then subtract it from the local time of the machine.
    /// The result is a time delta value, is seconds.
    pub(crate) async fn time_delta(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let server_time: u64 = self.get_noauth("/auth/time").await?.text().await?.parse()?;
        let delta = (now() - server_time).try_into()?;
        Ok(delta)
    }

    fn default_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "X-Ovh-Application",
            reqwest::header::HeaderValue::from_str(&self.application_key).unwrap(),
        );
        headers
    }

    async fn gen_headers(
        &self,
        url: &str,
        method: &str,
        body: &str,
    ) -> Result<HeaderMap, Box<dyn std::error::Error>> {
        let mut headers = self.default_headers();
        headers.insert(
            "Content-Type",
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let time_delta = self.time_delta().await?;
        let now: i64 = now().try_into()?;
        let timestamp = now + time_delta;
        let timestamp = timestamp.to_string();

        let signature = self.signature(url, &timestamp, method, body);

        insert_sensitive_header(&mut headers, "X-Ovh-Consumer", &self.consumer_key);
        insert_sensitive_header(&mut headers, "X-Ovh-Timestamp", &timestamp);
        insert_sensitive_header(&mut headers, "X-Ovh-Signature", &signature);

        Ok(headers)
    }

    /// Performs a GET request.
    pub(crate) async fn get(&self, path: &str) -> Result<Response, Box<dyn std::error::Error>> {
        let url = self.url(path);
        let headers = self.gen_headers(&url, "GET", "").await?;

        let resp = self.client.get(url).headers(headers).send().await?;
        Ok(resp)
    }

    /// Performs a DELETE request.
    pub(crate) async fn delete(&self, path: &str) -> Result<Response, Box<dyn std::error::Error>> {
        let url = self.url(path);
        let headers = self.gen_headers(&url, "DELETE", "").await?;

        let resp = self.client.delete(url).headers(headers).send().await?;
        Ok(resp)
    }

    /// Performs a POST request.
    pub(crate) async fn post<T: Serialize + ?Sized>(
        &self,
        path: &str,
        data: &T,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = self.url(path);

        // Cannot call RequestBuilder.json directly because of body
        // signature requirement.
        let body = serde_json::to_string(data)?;
        let headers = self.gen_headers(&url, "POST", &body).await?;

        let resp = self
            .client
            .post(url)
            .headers(headers)
            .body(body)
            .send()
            .await?;
        Ok(resp)
    }

    /// Performs a PUT request.
    pub(crate) async fn put<T: Serialize + ?Sized>(
        &self,
        path: &str,
        data: &T,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = self.url(path);

        // Cannot call RequestBuilder.json directly because of body
        // signature requirement.
        let body = serde_json::to_string(data)?;
        let headers = self.gen_headers(&url, "PUT", &body).await?;

        let resp = self
            .client
            .put(url)
            .headers(headers)
            .body(body)
            .send()
            .await?;
        if resp.status() != StatusCode::OK {
            return Err(Box::try_from("OVH API didn't returned status code 200").unwrap());
        }
        Ok(resp)
    }

    /// Performs a GET request without auth.
    pub(crate) async fn get_noauth(
        &self,
        path: &str,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = self.url(path);
        let headers = self.default_headers();

        let resp = self.client.get(url).headers(headers).send().await?;
        Ok(resp)
    }
}
