import os

import models.bert_hier
import models.bert_with_label
import models.t5_hier

from models.bert_hier import BertLocalEncoder
from rust_config import ExternalConfig

os.environ['TF_CPP_MIN_LOG_LEVEL'] = '3'
os.environ['WANDB_DISABLED'] = 'true'
from transformers import AutoTokenizer, AutoModelForCausalLM, Trainer, TrainingArguments, GPT2Config
from transformers import AutoModelForMaskedLM, AutoModelForQuestionAnswering, AutoModelForSequenceClassification
from transformers import AutoConfig
from transformers import T5ForConditionalGeneration

from external_dataset import ExternalDataset
import argparse
import subprocess
import multiprocessing as mp
import torch
import config_loader
from functools import partial


""" Basic 
"""

def run_loader(args):
    if args.cache is None or len(args.cache) == 0:
        subprocess.run(["cargo", "run", "--release", "--", "--task", args.task], cwd="../rust")
    else:
        print("Running with Cache", args.cache)
        subprocess.run(["cargo", "run", "--release", "--", "--task", args.task, '--cache', args.cache], cwd="../rust")



def run_model(args):

    tokenized_dataset = ExternalDataset("ipc:///tmp/masking_train")

    for data in tokenized_dataset:
        print("Rx Data")




parser = argparse.ArgumentParser(description='Run Model with External Data Loader')
parser.add_argument('--task', type=str, choices=["mlm", "clm", "span", "squad", "single", "multi", "python", "span-python"], default="mlm")
parser.add_argument('--all', action='store_true', default=False)
parser.add_argument('--cache', type=str, default=None)



def main():
    args = parser.parse_args()
    print(args)

    if args.all:
        pr = mp.Process(target=run_loader, args=(args,))
        pr.start()

    run_model(args)


if __name__ == '__main__':
    main()
