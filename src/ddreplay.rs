//
// Submissions to DDReplay
//

use std::sync::Arc;
use anyhow::Result;
use futures::StreamExt;
use hyper::{Client, Method, Body, Request};
use hyper_tls::HttpsConnector;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
struct DDReplayUploadRequest {
    data: String, // As base64
}

pub async fn upload_replay(replay: Arc<Vec<u8>>) -> Result<()> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let path = format!("upload");
    let uri = format!("https://ddreplay.herokuapp.com/{}", path);
    let req = DDReplayUploadRequest {
        data: base64::encode(&*replay),
    };
    let req = Request::builder()
        .header("content-type", "application/json")
        .header("accept", "application/json")
        .method(Method::POST)
        .uri(uri)
        .body(Body::from(serde_json::to_string(&req)?))
        .unwrap();
    let mut res = client.request(req).await?;
    let mut body = Vec::new();
    while let Some(chunk) = res.body_mut().next().await {
        body.extend_from_slice(&chunk?);
    }
    if res.status() != 200 {
        unsafe { anyhow::bail!(String::from_utf8_unchecked(body)); }
    }
    
    Ok(())
}