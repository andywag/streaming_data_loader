import zmq
import pickle
import time
import yaml
import argparse
import numpy as np


from transformers import  AutoModelForSequenceClassification, AutoConfig, TrainingArguments, Trainer, AutoTokenizer
from datasets import load_dataset

from external_dataset import ExternalDataset


def run_bert():
    #data = load_dataset("xed_en_fi", "en_annotated")
    
    #tokenized_dataset = data.map(tokenize_function, batched=True)

    dataset = ExternalDataset("ipc:///tmp/multi-label")

    config = AutoConfig.from_pretrained("bert-base-uncased")
    config.problem_type = "multi_label_classification"
    config.num_labels = 9
    
    training_args = TrainingArguments(output_dir = "local",
                                      learning_rate=1e-5,
                                      per_device_train_batch_size=32,
                                      logging_steps=100,
                                      num_train_epochs=1)
    model = AutoModelForSequenceClassification.from_config(config).train()


    trainer = Trainer(
        model=model,
        args=training_args,
        train_dataset=dataset,
        eval_dataset=dataset,
    )
    trainer.train()


parser = argparse.ArgumentParser(description='Test Data Loading')
parser.add_argument('--file', type=str, default='../rust/tests/masking_tests.yaml')
parser.add_argument('--config', type=str, default='zmq_ipc')
parser.add_argument('--iterations', type=int, default=5000)
parser.add_argument('--report', type=int, default=100)

def main():
    run_bert()

if __name__ == '__main__':
    main()



#test_transport("localhost",4000)
