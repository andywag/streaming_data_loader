import zmq
import pickle
import time
import yaml
import argparse
import numpy as np


from transformers import AutoModelForQuestionAnswering, AutoConfig, TrainingArguments, Trainer, AutoTokenizer
from datasets import load_dataset

def compute_metrics(eval_pred):
    logits, labels = eval_pred
    predictions = np.argmax(logits, axis=-1)
    return metric.compute(predictions=predictions, references=labels)


def tokenize_function(examples):
    def find_index(locations, index):
        s, e = 0, len(locations)
        while e > s:
            c = s + int((e-s)/2)
            cl = locations[c]
            if locations[c][0] == locations[c][1] == 0:
                if c == 0:
                    return 0
                e = c
            elif locations[c][0] <= index <= locations[c][1]:
                return c
            elif index < locations[c][0]:
                e = c-1
            else:
                s = c+1
        return s

    tokenizer = AutoTokenizer.from_pretrained('bert-base-uncased')
    data = tokenizer(examples["question"], examples["context"], padding="max_length", truncation=True, return_offsets_mapping=True)
    start = [0]*len(data['input_ids'])
    end = [0]*len(data['input_ids'])
    for x in range(len(data['input_ids'])):
        sp = examples['answers'][x]['answer_start'][0]
        ep = sp + len(examples['answers'][x]['text'][0])
        result = examples['context'][x][sp:ep]
        start[x] = find_index(data['offset_mapping'][x], sp)
        end[x] = find_index(data['offset_mapping'][x], ep)
        pass
    data['start_positions'] = start
    data['end_positions'] = end
    print(data[0])

    return data

def run_bert():
    data = load_dataset("../../datasets/squad")
    temp = data['train'][2]
    tokenized_dataset = data.map(tokenize_function, batched=True)
    config = AutoConfig.from_pretrained("bert-base-uncased")
    
    training_args = TrainingArguments(output_dir = "local")
    model = AutoModelForQuestionAnswering.from_config(config).train()


    trainer = Trainer(
        model=model,
        args=training_args,
        train_dataset=tokenized_dataset['train'],
        eval_dataset=tokenized_dataset['validation'],
        #compute_metrics=compute_metrics,
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
