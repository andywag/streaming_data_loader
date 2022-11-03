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
        result = np.zeros((len(labels), 9), dtype=np.float32)
        for x in range(len(labels)):
            for label in labels[x]:
                result[x][label] = 1.0
        return result

    tokenizer = AutoTokenizer.from_pretrained('bert-base-uncased')
    data = tokenizer(examples['sentence'], padding="max_length", truncation=True, max_length=96)
    data['labels'] = convert_labels(examples['labels']).tolist()
    return data


def compare_dataset():
    def check(a, b):
        an = np.asarray(a)
        bn = np.asarray(b)
        return np.array_equal(an,bn)

    data = load_dataset("xed_en_fi", "en_annotated")
    tokenized_dataset = data.map(tokenize_function, batched=True)
    external_dataset = ExternalDataset("ipc:///tmp/emot_python_compare")

    compare_items = ['input_ids', 'attention_mask', 'labels']
    for i, data in enumerate(tokenized_dataset['train']):
        external_data = next(external_dataset)
        pass
        for ci in compare_items:
            m = check(data[ci], external_data[ci])
            if not m:
                print("FAILED : Mismatch on line ", i)


parser = argparse.ArgumentParser(description='Test Data Loading')
parser.add_argument('--file', type=str, default='../rust/tests/masking_tests.yaml')
parser.add_argument('--config', type=str, default='zmq_ipc')
parser.add_argument('--iterations', type=int, default=5000)
parser.add_argument('--report', type=int, default=100)


def main():
    compare_dataset()


if __name__ == '__main__':
    main()

