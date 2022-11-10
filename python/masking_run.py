import zmq
import pickle
import time
import argparse
import numpy as np
import config_loader


from transformers import  AutoModelForMaskedLM, AutoConfig, TrainingArguments, Trainer, AutoTokenizer
from datasets import load_dataset

from external_dataset import ExternalDataset


def run_bert(iconfig):
    tokenizer_name = iconfig['tokenizer']['config']['tokenizer_name']
    sequence_length = iconfig['tokenizer']['config']['sequence_length']
    batch_size = iconfig['tokenizer']['config']['batch_size']

    def tokenize_function(examples):
        tokenizer = AutoTokenizer.from_pretrained(tokenizer_name)
        data = tokenizer(examples['text'], padding="max_length", truncation=True, max_length=sequence_length)
        return data

    tokenized_dataset = ExternalDataset(iconfig['transport']['transport'].address, batch_size, fields=["input_ids","attention_mask","labels"])

    config = AutoConfig.from_pretrained(tokenizer_name)

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

parser.add_argument('--file', type=str, default="../rust/tests/masking.yaml")
parser.add_argument('--config', type=str, default="zmq_pile_none")

def main():
    args = parser.parse_args()
    config_file = config_loader.load(args.file)
    config = config_file[args.config]
    run_bert(config)


if __name__ == '__main__':
    main()



