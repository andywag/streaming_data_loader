use std::path::PathBuf;

use super::{provider_config::Dataset, ProviderChannel, general_file_provider::Counter, cache_writer::CacheWriter, source_filter::SourceFilter};
use async_compression::tokio::bufread::GzipDecoder;
use tokio::{io::{AsyncBufReadExt, BufReader, Lines}, fs::File};

use tokio::sync::mpsc::Sender;
use futures::stream::TryStreamExt;
use tokio_util::compat::FuturesAsyncReadCompatExt;

 

pub async fn create_lines(file_path:&PathBuf) -> Option<Lines<BufReader<GzipDecoder<BufReader<File>>>>> {
    let file_opt = File::open(file_path).await;
    match file_opt {
        Ok(file) => {
            let reader = BufReader::new(file);
            let gzip_decoder = GzipDecoder::new(reader);
            let buf_reader = tokio::io::BufReader::with_capacity(100000, gzip_decoder);
            Some(buf_reader.lines())
        },
        Err(_) => {
            log::error!("File Not Found {:?}", file_path);
            None
        }
    }

}

pub async fn load_dataset(path:&PathBuf, counter:&mut Counter, tx:&Sender<ProviderChannel<String>>, filter:&SourceFilter) {
    let lines_opt = create_lines(path).await;
    if lines_opt.is_none() {
        return;
    }
    let mut lines = lines_opt.unwrap();
    while let Some(line) = lines.next_line().await.unwrap() {
        let text = filter.get_text(line);
        match text {
            Some(x) => {
                let _res_ = tx.send(ProviderChannel::Data(x)).await;
                if counter.inc_data() {
                    return;
                }
            },
            None => {
                continue
            },
        }
    }
}

pub async fn load_url(dataset:&Dataset, 
    counter:&mut Counter, 
    tx:&Sender<ProviderChannel<String>>, 
    mut cache_writer:Option<CacheWriter>,
    filter:&SourceFilter) {

    let response = reqwest::get(dataset.location.to_owned()).await.unwrap();
    let stream = response
        .bytes_stream()
        .map_err(|e| futures::io::Error::new(futures::io::ErrorKind::Other, e))
        .into_async_read()
        .compat();
    let gzip_decoder = GzipDecoder::new(stream);
    let buf_reader = tokio::io::BufReader::with_capacity(100000, gzip_decoder);
    let mut lines = buf_reader.lines();

    let mut error_count = 0;
    loop {
        let data = lines.next_line().await;
        match data {
            Ok(Some(line)) => {
                //let text = super::provider_util::create_json_text(line, "text");
                let text = filter.get_text(line);
                match text {
                    Some(x) => {
                        cache_writer.as_mut().map(|s| s.write_line(x.to_owned()));

                        let _res = tx.send(ProviderChannel::Data(x)).await;
                        if counter.inc_data() {
                            return;
                        }
                    },
                    None => {
                        continue
                    },
                }
            },
            Ok(None) => {
                continue;
            },
            Err(e) => {
                log::error!("Error in File Read {:?}", e);
                error_count += 1;
                if error_count == 3 {
                    return;
                }
            },
        }
    }
            
}



