import zmq
import pickle
import time
import yaml
import argparse
import numpy as np


from transformers import  AutoModelForSequenceClassification, AutoConfig, TrainingArguments, Trainer, AutoTokenizer
from datasets import load_dataset

from external_dataset import ExternalDataset


def tokenize_function(examples):
    def convert_labels(labels):
        data = np.zeros((len(labels), 9), dtype=np.float32)
        for x in range(len(labels)):
            for label in labels[x]:
                data[x][label] = 1.0000001
        return data

    tokenizer = AutoTokenizer.from_pretrained('bert-base-uncased')
    data = tokenizer(examples['sentence'], padding="max_length", truncation=True, max_length=96)

    labels = convert_labels(examples['labels'])
    data['labels'] = labels.tolist()
    return data


def run_bert(external=False):
    if not external:
        data = load_dataset("xed_en_fi", "en_annotated")
        tokenized_dataset = data.map(tokenize_function, batched=True)['train']
    else:
        tokenized_dataset = ExternalDataset("ipc:///tmp/multi-label")

    config = AutoConfig.from_pretrained("bert-base-uncased")
    config.problem_type = "multi_label_classification"
    config.num_labels = 9

    training_args = TrainingArguments(output_dir="local",
                                      lr_scheduler_type="constant",
                                      learning_rate=5e-6,
                                      warmup_steps=0.0,
                                      per_device_train_batch_size=32,
                                      logging_steps=8,
                                      num_train_epochs=6,
                                      save_steps=1000000,
                                      gradient_accumulation_steps=8
                                      )
    model = AutoModelForSequenceClassification.from_pretrained('bert-base-uncased',config=config).train()

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
