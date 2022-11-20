use crate::video_request_struct::{ClientRequest, Video};


pub async fn query_videos(msc_name:String) -> String{

    let query:String = format!("https://www.youtube.com/results?search_query={query}",query=msc_name);

    let mut local:ClientRequest = ClientRequest {
        client: reqwest::Client::new(),
        re_pat: regex::Regex::new(r"var ytInitialData =(.*?);</script>").unwrap(),
    };
   
    let html = local.get_html(query.to_owned()).await.unwrap();
    let data = local.get_data(html.to_owned()).await.unwrap();
    let json = local.get_json(data).await;
    let videos : Vec<Video>= local.get_results(json.to_owned()).await;
    

    let j = serde_json::to_string(&videos);

    match j {
        Ok(v) => v,
        Err(_) => String::from("")
    }
}
