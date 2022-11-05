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

    tokenizer = AutoTokenizer.from_pretrained('bert-base-uncased')
    data = tokenizer(examples['text'], padding="max_length", truncation=True, max_length=384)
    return data


def compare_dataset():
    def check(a, b):
        if type(a) != list:
            return a == b
        an = np.asarray(a)
        bn = np.asarray(b)
        if len(an) > 64:
            return np.array_equal(an[:-1], bn[:-1])
        else:
            return np.array_equal(an, bn)

    data = load_dataset("imdb")
    tokenized_dataset = data.map(tokenize_function, batched=True)
    external_dataset = ExternalDataset("ipc:///tmp/imdb_python_compare")

    compare_items = ['input_ids', 'attention_mask', 'label']
    for i, data in enumerate(tokenized_dataset['train']):
        external_data = next(external_dataset)

        pass
        for ci in compare_items:
            m = check(data[ci], external_data[ci])
            if not m:
                print("FAILED : Mismatch on line ", i, data[ci], external_data[ci])

def main():
    compare_dataset()


if __name__ == '__main__':
    main()

