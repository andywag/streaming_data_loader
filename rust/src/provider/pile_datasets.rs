use serde::{Deserialize, Serialize};

use super::{provider_config::Dataset};
use clap;

#[derive(Deserialize, Serialize, Debug, clap::ValueEnum, Clone)]

pub enum PileDatasetType {
    #[serde(rename="none")]
    None,
    #[serde(rename="config")]
    Config,
    #[serde(rename="total")]
    Total,
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
    USPTODataset, 
    #[serde(rename="euro")]
    EuroParlDataset, 
    #[serde(rename="ytsub")]
    YTSubtitlesDataset, 
    #[serde(rename="hacker")]
    HackerNewsDataset , 
    #[serde(rename="git_full")]
    FullGithubDataset, 
    #[serde(rename="git_small")]
    GithubDataset, 
    #[serde(rename="openweb")]
    OpenWebText2Dataset, 
    #[serde(rename="common_crawl")]
    CommonCrawlDataset,  
}


fn get_internal_zstd(location:String, exists:bool) -> Option<Vec<Dataset>> {
    if !exists {
        None
    } else {
        let dataset = Dataset{
            location: location
        };
        Some(vec!(dataset))
    }
}

pub fn get_datasets(typ:PileDatasetType) -> Option<Vec<Dataset>> {
    match typ {
        PileDatasetType::None => None,
        PileDatasetType::Config => None,
        PileDatasetType::Total => {
            let mut result = Vec::<Dataset>::with_capacity(30);
            for x in 0..30 {
                let location = if x < 10 {format!("https://mystic.the-eye.eu/public/AI/pile/train/0{}.jsonl.zst",x)}
                               else {format!("https://mystic.the-eye.eu/public/AI/pile/train/{}.jsonl.zst",x)};
                let dataset = Dataset{
                    location: location,
                };
                result.push(dataset);
            }
            Some(result)
            
        },
        PileDatasetType::Wiki => {
            let _dataset = Dataset{
                location: "http://eaidata.bmk.sh/data/wikipedia-en.tar.gz".to_string(),
            };
            None
            //vec!(dataset)
        },
        PileDatasetType::OpensubtitlesDataset => {
            let _dataset = Dataset{
                location: "http://eaidata.bmk.sh/data/opensubtitles_out.tar".to_string(),
            };
            None
            //vec!(dataset)
        },
        PileDatasetType::BookCorpus => {
            let _dataset = Dataset{
                location: "https://the-eye.eu/public/AI/pile_preliminary_components/books1.tar.gz".to_string(),
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


