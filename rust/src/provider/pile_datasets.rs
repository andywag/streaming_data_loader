use serde::{Deserialize, Serialize};

use super::{Dataset, DownloadType};

#[derive(Deserialize, Serialize, Debug)]

pub enum PileDatasetType {
    #[serde(rename="total")]
    TOTAL,
    #[serde(rename="wiki")]
    Wiki,
    #[serde(rename="subtitles")]
    OpensubtitlesDataset,
    #[serde(rename="book")]
    BookCorpus,
    #[serde(rename="enron")]
    Enron,
    #[serde(rename="literotica")]
    Literotica,
    #[serde(rename="bibliotik")]
    Bibliotik,
    #[serde(rename="ubuntu")]
    UbuntuIRCDataset,
    #[serde(rename="arxiv")]
    ArXiv,
    #[serde(rename="pubmed")]
    PubMedDataset,
    #[serde(rename="exporter")]
    ExPorterDataset,
    #[serde(rename="stack")]
    StackExchangeDataset,
    #[serde(rename="freelaw")]
    FreeLawDataset,
    #[serde(rename="pubmedcentral")]
    PubMedCentral,
    #[serde(rename="philpapers")]
    PhilPapersDataset,
    #[serde(rename="uspto")]
    USPTODataset, //'https://the-eye.eu/public/AI/pile_preliminary_components/pile_uspto.tar'
    #[serde(rename="euro")]
    EuroParlDataset, // 'https://the-eye.eu/public/AI/pile_preliminary_components/EuroParliamentProceedings_1996_2011.jsonl.zst'
    #[serde(rename="ytsub")]
    YTSubtitlesDataset, //'http://eaidata.bmk.sh/data/yt_subs.jsonl.zst'
    #[serde(rename="hacker")]
    HackerNewsDataset , //'http://eaidata.bmk.sh/data/hn.jsonl.zst'
    #[serde(rename="git_full")]
    FullGithubDataset, //'http://eaidata.bmk.sh/data/github.tar'
    #[serde(rename="git_small")]
    GithubDataset, //'http://eaidata.bmk.sh/data/github_small.jsonl.zst'),
    #[serde(rename="openweb")]
    OpenWebText2Dataset, //'http://eaidata.bmk.sh/data/openwebtext2.jsonl.zst.tar')
    #[serde(rename="common_crawl")]
    CommonCrawlDataset,  //'http://eaidata.bmk.sh/data/pile_cc_filtered_deduped.jsonl.zst'),
}


fn get_internal_zstd(location:String, exists:bool) -> Option<Vec<Dataset>> {
    if !exists {
        None
    } else {
        let dataset = Dataset{
            location: location,
            download_type: DownloadType::Zstd,
            network: true,
        };
        Some(vec!(dataset))
    }
}

pub fn get_datasets(typ:PileDatasetType) -> Option<Vec<Dataset>> {
    match typ {
        PileDatasetType::TOTAL => {
            let mut result = Vec::<Dataset>::with_capacity(30);
            for x in 0..30 {
                let location = if x < 10 {format!("https://mystic.the-eye.eu/public/AI/pile/train/0{}.jsonl.zst",x)}
                               else {format!("https://mystic.the-eye.eu/public/AI/pile/train/{}.jsonl.zst",x)};
                let dataset = Dataset{
                    location: location,
                    download_type: DownloadType::Zstd,
                    network: true,
                };
                result.push(dataset);
            }
            Some(result)
            
        },
        PileDatasetType::Wiki => {
            let _dataset = Dataset{
                location: "http://eaidata.bmk.sh/data/wikipedia-en.tar.gz".to_string(),
                download_type: DownloadType::Gzip,
                network: true,
            };
            None
            //vec!(dataset)
        },
        PileDatasetType::OpensubtitlesDataset => {
            let _dataset = Dataset{
                location: "http://eaidata.bmk.sh/data/opensubtitles_out.tar".to_string(),
                download_type: DownloadType::Gzip,
                network: true,
            };
            None
            //vec!(dataset)
        },
        PileDatasetType::BookCorpus => {
            let _dataset = Dataset{
                location: "https://the-eye.eu/public/AI/pile_preliminary_components/books1.tar.gz".to_string(),
                download_type: DownloadType::Gzip,
                network: true,
            };
            None
            //vec!(dataset)
        },
        PileDatasetType::Enron => {
            get_internal_zstd("http://eaidata.bmk.sh/data/enron_emails.jsonl.zst".to_string(), true)
        },
        PileDatasetType::Literotica => {
            get_internal_zstd("https://the-eye.eu/public/AI/pile_preliminary_components/Literotica.jsonl.zst".to_string(), true)
        },
        PileDatasetType::Bibliotik => {
            get_internal_zstd("https://the-eye.eu/public/AI/pile_preliminary_components/Literotica.jsonl.zst".to_string(), false)
        },
        PileDatasetType::UbuntuIRCDataset => {
            get_internal_zstd("http://eaidata.bmk.sh/data/ubuntu_irc_weekly.jsonl.zst".to_string(), true)
        },
        PileDatasetType::ArXiv => {
            get_internal_zstd("http://eaidata.bmk.sh/data/arxiv.jsonl.zst".to_string(), true)
        },
        PileDatasetType::PubMedDataset => {
            get_internal_zstd("https://the-eye.eu/public/AI/pile_preliminary_components/PUBMED_title_abstracts_2019_baseline.jsonl.zst".to_string(), true)       
        },
        PileDatasetType::ExPorterDataset => {
            get_internal_zstd("https://the-eye.eu/public/AI/pile_preliminary_components/NIH_ExPORTER_awarded_grant_text.jsonl.zst".to_string(), true)       
        },
        PileDatasetType::StackExchangeDataset => {
            // TODO : Needs tar support
            get_internal_zstd("http://eaidata.bmk.sh/data/stackexchange_dataset.tar".to_string(), true)       
        },
        PileDatasetType::FreeLawDataset => {
            // TODO : Needs tar support
            get_internal_zstd("https://the-eye.eu/public/AI/pile_preliminary_components/FreeLaw_Opinions.jsonl.zst".to_string(), true)       
        }
        PileDatasetType::PubMedCentral => {
            // TODO : Needs tar.gz
            get_internal_zstd("https://the-eye.eu/public/AI/pile_preliminary_components/PMC_extracts.tar.gz".to_string(), true)       
        }
        PileDatasetType::PhilPapersDataset => {
            // TODO : Needs tar.gz
            get_internal_zstd("https://the-eye.eu/public/AI/pile_preliminary_components/PhilArchive.jsonl.zst".to_string(), true)       
        }
        PileDatasetType::USPTODataset => {
            // Needs Tar File
            get_internal_zstd("https://the-eye.eu/public/AI/pile_preliminary_components/pile_uspto.tar".to_string(), false)       
        }, 
        PileDatasetType::EuroParlDataset => {
            get_internal_zstd("https://the-eye.eu/public/AI/pile_preliminary_components/EuroParliamentProceedings_1996_2011.jsonl.zst".to_string(), true)       
        }, 
        PileDatasetType::YTSubtitlesDataset => {
            get_internal_zstd("http://eaidata.bmk.sh/data/yt_subs.jsonl.zst".to_string(), true)       
        }, 
        PileDatasetType::HackerNewsDataset => {
            get_internal_zstd("http://eaidata.bmk.sh/data/hn.jsonl.zst".to_string(), true)       
        },  
        PileDatasetType::FullGithubDataset => {
            // TODO : Needs Tar File Support
            get_internal_zstd("http://eaidata.bmk.sh/data/github.tar".to_string(), true)       
        },   
        PileDatasetType::GithubDataset => {
            get_internal_zstd("http://eaidata.bmk.sh/data/github_small.jsonl.zst".to_string(), true)       
        },   
        PileDatasetType::OpenWebText2Dataset => {
            get_internal_zstd("http://eaidata.bmk.sh/data/openwebtext2.jsonl.zst.tar".to_string(), true)       
        },  
        PileDatasetType::CommonCrawlDataset => {
            get_internal_zstd("http://eaidata.bmk.sh/data/pile_cc_filtered_deduped.jsonl.zst".to_string(), true)       
        },  
        
    }
}



#[derive(Deserialize, Serialize, Debug)]
pub struct WikiDataset {
    dataset:Vec<Dataset>
}

impl WikiDataset {
    pub fn new() -> Self {
        Self {
            dataset : vec!(Dataset{location:"http://eaidata.bmk.sh/data/wikipedia-en.tar.gz".to_string() , 
                download_type:DownloadType::Gzip,
                network:true})
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]

pub enum PileSets {
    Wiki(WikiDataset)
}