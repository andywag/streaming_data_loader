# data_loader

The purpose of this project is to provide a streaming dataloader for NLP which runs independently of the model. The project currently supports training but will also handle inference longer term. The motivation behind this was based on solving a few issues : 

1. Issues with Python Performance Related to PreProcessing data
2. Support for handling Streaming Interfaces
3. Quick Startup to Avoid File Downloading and preprocessing

## Quick Start

1. Install Rust **curl https://sh.rustup.rs -sSf | sh**
2. Install Libraries **sudo apt-get install libzmq3-dev pkg-config  libssl-dev**
3. Build Rust Libary **cargo build --release** from rust folder -- Might take a while on first build
4. Run **python3 top_run.py --task mlm --all** from python folder -- What's Masked LM training using Pile Dataset

These instructions will run BERT pretraining using the Pile dataset and will avoid all the pain associated preprocessing datasets normally associated with pretraining. 

## Dataset support

The data loader is general but directly supports : 

1. Wikipedia data from https://dumps.wikimedia.org/
2. Data stored in an Arror format
    a. Streaming support from HuggingFace Datasets

## Status

This project is still a work in progress and in a relatively early stage. Basic functionallity for training is working with examples for masking and single and multi label sequence classification. Current focus is on greater generalization and ease of use along with bug fixing. The hooks exist to allow **relatively easy** addition of new tasks/datasets. 

## Operation/Examples

There are currently a few example cases which are directly supported and can be run using : 

1. **python3 top_run.py --task <task> --all** from the python folder
    a. task = ["mlm" (Masked Language Model), "clm" (GPT), "span" (T5), "multi-label"]


### Masking/Pretraining

This example will run pretraining using a wiki dataset which is streamed from https://dumps.wikimedia.org/other/cirrussearch/current/enwiki-20221021-cirrussearch-content.json.gz. There are a few known issues with this dataset but it is converging over the initial iterations. The training for this will start immediately without the initial required preprocessing which makes pretraining so painful. 

1. Run **python3 top_run.py --task mlm --all** from python folder

Configuration and Source Code for this Example can be found
* https://github.com/andywag/streaming_data_loader/tree/master/rust/src/tasks/masking

### Multi Label Classification

This example will run multilabel classification using an emotions dataset downloaded from Huggingface. The first step of the process is to download the arrow dataset so the first time this example is run will have a slight delay while this is downloaded. Arrow also supports a streaming mode which is being worked on now. 

1. Run **python3 top_run.py --task emot --all** from python folder


Configuration and Source Code for this Example can be found
* https://github.com/andywag/streaming_data_loader/tree/master/rust/src/tasks/multi_label


## Architecture

The goal of this design is to allow easy addition of tasks and datasets with a basic API. The API is in heavy flux right now but examples of how to add tasks can be seen in the rust/tasks folder. There are 4 basic services that need to be configured. In general, addition of a dataset only requires 

1. Specifying the format of the input file
2. Specifying the processing to convert the input data to the proper format for the input model

The project splits the training creation into 4 separate services which communicate based on rust channels. The services are : 

1. Data Provider : Downloads Data from Internet or Local File and Streams the Data to the Batcher
2. Batcher : Preprocesses the data (tokenizes for NLP) and puts the data into a batch
3. Transport : Transports the Data to the Device Performing the Training (Currently Only Supports ZMQ)
4. Device : Runs the Training


## Testing

The tests download datasets from huggingface using python which unfortunately takes time during the first run. This may appear like a hang so initial testing might be slow. After initial downloads testing is relatively fast. 





