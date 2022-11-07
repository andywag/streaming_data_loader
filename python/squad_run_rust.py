import zmq
import pickle
import time
import yaml
import argparse
import numpy as np


from transformers import  AutoModelForQuestionAnswering, AutoConfig, TrainingArguments, Trainer, AutoTokenizer
from datasets import load_dataset

from external_dataset import ExternalDataset


def tokenize_function(examples):

    tokenizer = AutoTokenizer.from_pretrained('bert-base-uncased')
    data = tokenizer(examples['text'], padding="max_length", truncation=True, max_length=96)
    return data

def run_bert(external=True):
    if not external:
        data = load_dataset("imdb")
        tokenizer = AutoTokenizer.from_pretrained('bert-base-uncased')
        tokenized_dataset = data.map(tokenize_function, batched=True)['train']
    else:
        tokenized_dataset = ExternalDataset("ipc:///tmp/squad")

    config = AutoConfig.from_pretrained("bert-base-uncased")
    config.problem_type = "single_label_classification"
    config.num_labels = 2

    training_args = TrainingArguments(output_dir="local",
                                      lr_scheduler_type="constant",
                                      learning_rate=1e-5,
                                      warmup_steps=0.0,
                                      per_device_train_batch_size=8,
                                      logging_steps=8,
                                      num_train_epochs=6,
                                      save_steps=1000000,
                                      gradient_accumulation_steps=8
                                      )
    model = AutoModelForQuestionAnswering.from_pretrained('bert-base-uncased',config=config).train()

    trainer = Trainer(
        model=model,
        args=training_args,
        train_dataset=tokenized_dataset,
        eval_dataset=tokenized_dataset,
    )
    trainer.train()


parser = argparse.ArgumentParser(description='Test Data Loading')
parser.add_argument('--rust', action='store_true')

def main():
    args = parser.parse_args()
    run_bert(args.rust)

if __name__ == '__main__':
    main()



#test_transport("localhost",4000)
