use serde_json::Value;
use std::env::{args, Args};
use reqwest::Client;
use regex::Regex;

use std::{env, io::Error, io::ErrorKind};


#[derive(Debug)]
enum QResults {
    Video(Vec<Video>),
}


#[derive(Debug)]
struct Video {
    id:String,
    title:String,
    thumbnail_url:String,
    channel_name: String,
    duration:String
}

#[derive(Debug)]
struct ClientRequest {
    client: Client,
    re_pat: regex::Regex,
}


impl ClientRequest{
    async fn get_html(&mut self, query:String) -> Result<String, Error>{
       let res = self
        .client
        .get(query)
        .send()
        .await;
        
        match res {
            Ok(val) => match val.text().await{
                Ok(w) => Ok(w),
                Err(_) => Err(Error::new(ErrorKind::Other, "Failed to read HTML")),
            },
            Err(_)=>Err(Error::new(ErrorKind::Other, "Failed to load page")),
        }
    }
    async fn get_data(&self, html: String) -> Result<String, Error> {
        let re = self.re_pat.captures(&html);
        match re {
            Some(v) => match v.get(1) {
                Some(w) => Ok(w.as_str().to_owned()),
                None => Err(Error::new(ErrorKind::Other, "Bad HTML")),
            },
            None => Err(Error::new(ErrorKind::Other, "Failed to match")),
        }
    }
    async fn get_json(&mut self, json_str: String) -> String {
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

    async fn get_results(&self, json: String, query_results: QResults) -> QResults {
        let reparse: Vec<Value> = serde_json::from_str(&json).unwrap();
        match query_results {
            QResults::Video(mut v) => { 
                for data in reparse {
                    if data["videoRenderer"].is_null() {
                        continue;
                    }
                    let uniform = &data["videoRenderer"];
                    // println!("{:?}", uniform); 
                    v.push(Video {
                        
                        
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
                QResults::Video(v)
            },
        }
    }
}

async fn query_videos(msc_name:String) -> QResults {

    let query:String = format!("https://www.youtube.com/results?search_query={query}",query=msc_name);
    let mut local:ClientRequest = ClientRequest { 
        client: Client::new(), 
        re_pat: Regex::new(r"var ytInitialData =(.*?);</script>").unwrap(), 
    };

    let html = local.get_html(query.to_owned()).await.unwrap();
    let data = local.get_data(html.to_owned()).await.unwrap();
    let json = local.get_json(data).await;
    
    let videos = local.get_results(json.to_owned(), QResults::Video(Vec::new())).await;

    videos
}

#[tokio::main]
async fn main() {
    let mut inputs = args();
    if inputs.len() <= 1 {
        return ()
    }

    let music_name:String = inputs.nth(1).unwrap();
    let videos = query_videos(music_name).await;

    println!("{:?}", videos);
}

