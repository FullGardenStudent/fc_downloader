
use std::fs::File;
use std::fs;
use std::io::{copy, Cursor};
use colored::Colorize;
use std::time::Instant;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let url = match std::env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("No CLI URL provided, using default.");
            "https://hyper.rs".into()
        }
    };

    let v : Vec<&str> = url.split('/').collect();

    let mut murl :String= "http://a.4cdn.org/".to_owned();
    murl.push_str(v[3]);
    murl.push_str("/thread/");
    murl.push_str(v[5]);
    murl.push_str(".json");
    let res = reqwest::get(&murl).await?.json::<serde_json::Value>().await?;
    let obj = res.get("posts").unwrap();
    let mut i:usize =0;
    let mut j:usize = 0;
    let bfilename:bool = true;
    while obj.get(i).is_some() {
        let arr = obj.get(i).expect("empty media data found!");
        if !arr["tim"].is_null(){
            j=j+1;
        }
        i=i+1;
    }

    println!("{} {} {} {} {}",
            format!("Fround").green().bold(),j,
            format!("media files out of").green().bold(),i,
            format!("posts in the thread!").green().bold());
    if bfilename {
        println!("{}",format!("Downloading media with their original file name...").green());
    }
    
    let start = Instant::now();
    let mut k:usize = 0;

    let mut path = "Downloads/".to_owned();
            path.push_str(v[3]);
            let b : bool = Path::new(&path).is_dir();
            if !b {
                fs::create_dir(&path).unwrap();
            }
            path.push_str("/");
            path.push_str(v[5]);
            let c = Path::new(&path).is_dir();
            if !c {
                fs::create_dir(&path).unwrap();
            }
            path.push_str("/");
    
    while obj.get(k).is_some(){
        let arr = obj.get(k).expect("empty media data found!");
        if !arr["tim"].is_null(){
            let tim = arr["tim"].to_string();
            let ext = arr["ext"].as_str().unwrap();
            let mut endurl : String = "http://i.4cdn.org/".to_owned();
            endurl.push_str(v[3]);
            endurl.push_str("/");
            endurl.push_str(&tim);
            endurl.push_str(ext);
            let mut filename : String;
            if bfilename {
                let f = arr["filename"].as_str().unwrap();
                filename= f.to_owned();
                filename.push_str(ext);
            }else{
                filename = tim.to_owned();
                filename.push_str(ext);
            }
            
            let fpath= path.to_string() + &filename;
            

            let d = Path::new(&fpath).exists();
            if d {
                println!("{} {} File {} already exists!",k,format!("->").blue(),filename.cyan());
            }else {
            let mut dest = File::create(&fpath).unwrap();
            let reesponse = reqwest::get(&endurl).await?;
            
            let mut content = Cursor::new(reesponse.bytes().await?);
            copy(&mut content, &mut dest).unwrap();
            println!("{} {} {}",k,format!("->").blue(),filename.cyan());
            }
        }
        k=k+1;
    }
    let end = start.elapsed();
    println!("{}",format!("Downloaded all available media!").italic().yellow().bold());
    println!("Execution time : {} seconds",end.as_secs_f32());
    //println!("Done!");

    Ok(())

}