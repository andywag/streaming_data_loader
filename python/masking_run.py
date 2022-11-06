import zmq
import pickle
import time
import yaml
import argparse
import numpy as np


from transformers import  AutoModelForMaskedLM, AutoConfig, TrainingArguments, Trainer, AutoTokenizer
from datasets import load_dataset

from external_dataset import ExternalDataset


def run_bert(external=True, tokenizer_name='bert-base-uncased'):
    def tokenize_function(examples):
        tokenizer = AutoTokenizer.from_pretrained(tokenizer_name)
        data = tokenizer(examples['text'], padding="max_length", truncation=True, max_length=96)
        return data

    tokenized_dataset = ExternalDataset("ipc:///tmp/masking_none", 1024, fields=["input_ids","attention_mask","labels"])

    config = AutoConfig.from_pretrained(tokenizer_name)
    #config.problem_type = "single_label_classification"
    #config.num_labels = 2

    training_args = TrainingArguments(output_dir="local",
                                      lr_scheduler_type="constant",
                                      learning_rate=1e-5,
                                      warmup_steps=0.0,
                                      per_device_train_batch_size=16,
                                      logging_steps=8,
                                      num_train_epochs=6,
                                      save_steps=1000000,
                                      gradient_accumulation_steps=8
                                      )
    model = AutoModelForMaskedLM.from_config(config=config).train()

    trainer = Trainer(
        model=model,
        args=training_args,
        train_dataset=tokenized_dataset,
        eval_dataset=tokenized_dataset,
    )
    trainer.train()


parser = argparse.ArgumentParser(description='Test Data Loading')
parser.add_argument('--tokenizer', type=str, default="bert-base-uncased")
parser.add_argument('--rust', action='store_true')

def main():
    args = parser.parse_args()
    run_bert(args.rust, args.tokenizer)

if __name__ == '__main__':
    main()



#test_transport("localhost",4000)
