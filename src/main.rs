use std::env::{args};

mod video_func;
mod video_request_struct;

use video_func::query_videos;
#[tokio::main]
async fn main() {

    //Cli things
    let mut inputs = args();
    if inputs.len() <= 1 {
        return ()
    }
    let music_name:String = inputs.nth(1).unwrap();
    //---
    
    let videos = query_videos(music_name).await;  
    println!("{:?}", videos);
}

