
use async_compression::tokio::bufread::GzipDecoder;
use futures::stream::TryStreamExt;
use tokio::{io::{AsyncBufReadExt, BufReader, Lines}, fs::File};
use tokio_util::compat::FuturesAsyncReadCompatExt;

use tokio::sync::mpsc::Sender;

use super::ProviderChannel;
 

fn create_text(line:String, finder:&str) -> String{
    let search:Vec<char> = finder.chars().collect();
    let mut index:usize = 0;
    let mut sp:usize = 0;
    let mut escape = false;
    let mut new_text = Vec::<char>::with_capacity(2048);
    let mut wait = 0;
    for (i,c) in line.chars().enumerate() {
        if sp == 0 {
            if c == search[index] {
                index += 1;
                if index == search.len() {
                    sp = i;
                }
            }
            else {
                index = 0;
            }
        }
        else {
            if sp != 0 && c == '"' && !escape {
                break;
            }
            else {
                if escape && c == 'u' {
                    wait = 4;
                }
                else if wait > 0 {
                    wait = wait - 1;
                }
                if c != '\\' && wait == 0{
                    new_text.push(c);
                }
            }
        }
        escape = c == '\\';
    }
    //println!("Here {} {}", sp, ep);
    return new_text.into_iter().collect();

}


pub async fn create_url_lines(file_path:&String) -> Lines<BufReader<GzipDecoder<BufReader<File>>>> {
    let file = File::open(file_path).await.unwrap();
    let reader = BufReader::new(file);
    let gzip_decoder = GzipDecoder::new(reader);
    let buf_reader = tokio::io::BufReader::with_capacity(100000, gzip_decoder);
    let lines = buf_reader.lines();
    return lines;
}

// TODO : Add Support for Limiting Iterations
pub async fn load_url(url:&String, iterations:u64, tx:Sender<ProviderChannel<String>>) {
    let response = reqwest::get(url).await.unwrap();

    let stream = response
        .bytes_stream()
        .map_err(|e| futures::io::Error::new(futures::io::ErrorKind::Other, e))
        .into_async_read()
        .compat();
    let gzip_decoder = GzipDecoder::new(stream);

    // Print decompressed txt content
    //let buf_reader = tokio::io::BufReader::new(gzip_decoder);
    let buf_reader = tokio::io::BufReader::with_capacity(100000, gzip_decoder);

    let mut lines = buf_reader.lines();
    let mut data_count = 0;
    loop {
        
        let mut index = 0;
        while let Some(line) = lines.next_line().await.unwrap() {
            if index % 2 == 1 {
                let text = create_text(line, "\"text\":\"");
                let res = tx.send(ProviderChannel::Data(text));
                let _result = res.await;
                //result.err().map(|e| println!("Error {:?}", e));
                if data_count == iterations {
                    let _result = tx.send(ProviderChannel::Complete).await;
                    println!("Finished with Data");
                    return;
                }
                data_count += 1;
            }
            index += 1;   
        }
        
        
    }

}

pub async fn create_lines(file_path:&String) -> Lines<BufReader<GzipDecoder<BufReader<File>>>> {
    let file = File::open(file_path).await.unwrap();
    let reader = BufReader::new(file);
    let gzip_decoder = GzipDecoder::new(reader);
    let buf_reader = tokio::io::BufReader::with_capacity(100000, gzip_decoder);
    let lines = buf_reader.lines();
    return lines;
}

pub async fn load_data(file_path:&String, iterations:u64, tx:Sender<ProviderChannel<String>>) {

    let mut lines = create_lines(file_path).await;

    let mut data_count = 0;
    loop {
        
        let mut index = 0;
        while let Some(line) = lines.next_line().await.unwrap() {
            if index % 2 == 1 {
                let text = create_text(line, "\"text\":\"");
                let res = tx.send(ProviderChannel::Data(text));
                let _result = res.await;
                //result.err().map(|e| println!("Error {:?}", e));
                if data_count == iterations {
                    let _result = tx.send(ProviderChannel::Complete).await;
                    println!("Finished with Data");
                    return;
                }
                data_count += 1;
            }
            index += 1;   
        }
        lines = create_lines(file_path).await;
        
    }

}
