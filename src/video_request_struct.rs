use reqwest::Client;
use regex::Regex;

use std::{io::Error, io::ErrorKind};
use serde_json::Value;


#[derive(Debug)]
pub struct Video {
    pub id: String,
    pub title: String,
    pub thumbnail_url: String,
    pub channel_name: String,
    pub duration: String,
}

#[derive(Debug)]
pub struct ClientRequest {
    pub client: Client,
    pub re_pat: Regex,
}


impl ClientRequest {
    pub(crate) async fn get_html(&mut self, query: String) -> Result<String, Error> {
        let res = self
            .client
            .get(query)
            .send()
            .await;

        match res {
            Ok(val) => match val.text().await {
                Ok(w) => Ok(w),
                Err(_) => Err(Error::new(ErrorKind::Other, "Failed to read HTML")),
            },
            Err(_) => Err(Error::new(ErrorKind::Other, "Failed to load page")),
        }
    }
    pub(crate) async fn get_data(&self, html: String) -> Result<String, Error> {
        let re = self.re_pat.captures(&html);
        match re {
            Some(v) => match v.get(1) {
                Some(w) => Ok(w.as_str().to_owned()),
                None => Err(Error::new(ErrorKind::Other, "Bad HTML")),
            },
            None => Err(Error::new(ErrorKind::Other, "Failed to match")),
        }
    }
    pub(crate) async fn get_json(&mut self, json_str: String) -> String {
        let parse: Value = serde_json::from_str(&json_str).unwrap();

        let req_parse: Value = serde_json::from_str(&format!(
            "{}",
            parse["contents"]["twoColumnSearchResultsRenderer"]["primaryContents"]
                ["sectionListRenderer"]["contents"][0]["itemSectionRenderer"]["contents"]
                .to_owned()
        ))
            .unwrap();
        format!("{}", req_parse)
    }

    pub(crate) async fn get_results(&self, json: String) -> Vec<Video> {
        let reparse: Vec<Value> = serde_json::from_str(&json).unwrap();
        let mut videos: Vec<Video> = Vec::new();

        for data in reparse {
            if data["videoRenderer"].is_null() {
                continue;
            }
            let uniform = &data["videoRenderer"];
            videos.push(Video {
                id: uniform["videoId"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                title: uniform["title"]["runs"][0]["text"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                thumbnail_url: uniform["thumbnail"]["thumbnails"][0]["url"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                channel_name: uniform["shortBylineText"]["runs"][0]["text"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                duration: uniform["lengthText"]["simpleText"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
            });
        }
        videos
    }
}

