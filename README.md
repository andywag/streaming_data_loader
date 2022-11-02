# data_loader

The purpose of this project is to provide a streaming dataloader for NLP which runs independently of the project. The project currently supports training but will also server inference longer term. The motivation behind this was based on solving a few issues : 

1. Issues with Python Performance Related to PreProcessing data
2. Support for handling Streaming Interfaces
3. Quick Startup to Avoid File Downloading and preprocessing

## Status

This project is still a work in progress and in a relatively early stage. The project splits the training creation into 4 separate items services which communicate based on rust channels. The services are : 

1. Data Provider : Downloads Data from Internet or Local File and Streams the Data to the Batcher
2. Batcher : Preprocesses the data (tokenizes for NLP) and puts the data into a batch
3. Transport : Transports the Data to the Device Performing the Training (Currently Only Supports ZMQ)
4. Device : Runs the Training

## Operation

An example of this operation can be seen with multi-label bert training using an emotion dataset. There are some issues with this process but the data flow is currently functional. To run this case : 

1. Run cargo run "--release -- --config zmq_ipc" in the rust folder
2. In parallel run "python3 emot_run_rust.py" in the python folder





