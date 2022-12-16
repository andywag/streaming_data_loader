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
    batch_size = tokenized_dataset.batch_size
    sequence_length = tokenized_dataset.context['batch']['sequence_length']
    config = ExternalConfig(tokenized_dataset.context)

    train_batch_size = 32
    gradient_accumulation = 32
    num_train_epochs = 1
    # Get the Common Configuration
    model_name = "bert-base-uncased"
    if args.task == 'mlm':
        model_name = "bert-base-uncased"
    elif args.task == 'clm':
        model_name = "gpt2"
    elif args.task == 'span':
        model_name = "t5-small"
    elif args.task == 'span-python':
        model_name = "t5-small"


    learning_rate = 1e-5
    if args.task == 'mlm':
        config = AutoConfig.from_pretrained(model_name)
        model = AutoModelForMaskedLM.from_config(config=config).train()
    elif args.task == 'python':
        config = AutoConfig.from_pretrained(model_name)
        config.num_hidden_layers = 12
        config.vocab_size = 4096
        config.max_position_embeddings = 512
        train_batch_size = 6

        config.context_layers = [2, 5, 7, 9, 11]
        model = AutoModelForMaskedLM.from_config(config=config).train()
        #model = models.bert_with_label.BertForMaskedLM(config).train()


        model.bert.encoder = BertLocalEncoder(config)
        model.bert.get_extended_attention_mask = models.bert_hier.get_extended_attention_mask

        #cp = "../../datasets/checkpoints/checkpoint-9000/pytorch_model.bin"
        #model.load_state_dict(torch.load(cp))




        gradient_accumulation = 4
        learning_rate = 2.0e-5

    elif args.task == 'clm':
        config = GPT2Config.from_pretrained(model_name)
        model = AutoModelForCausalLM.from_config(config=config).train()
        train_batch_size = 8
    elif args.task == 'squad':
        config = AutoConfig.from_pretrained("bert-base-uncased")
        model = AutoModelForQuestionAnswering.from_pretrained("bert-base-uncased",config=config).train()
        train_batch_size = 8
        learning_rate = 1e-5
        gradient_accumulation = 32
    elif args.task == 'single':
        config = AutoConfig.from_pretrained("bert-base-uncased")
        config.problem_type = "single_label_classification"
        config.num_labels = 2
        gradient_accumulation = 1
        model = AutoModelForSequenceClassification.from_pretrained("bert-base-uncased",config=config).train()
    elif args.task == 'multi':
        config = AutoConfig.from_pretrained("bert-base-uncased")
        config.problem_type = "multi_label_classification"
        config.num_labels = 9
        learning_rate = 1e-5
        num_train_epochs = 3
        gradient_accumulation = 2

        model = AutoModelForSequenceClassification.from_pretrained("bert-base-uncased",config=config).train()


    elif args.task == 'span':
        config = AutoConfig.from_pretrained(model_name)
        model = T5ForConditionalGeneration.from_pretrained(model_name, config=config).train()

    elif args.task == 'span-python':
        config = AutoConfig.from_pretrained(model_name)
        config.vocab_size = 4096
        #model = T5ForConditionalGeneration(config=config).train()
        model = models.t5_hier.T5ForConditionalGeneration(config=config).train()

        for x in range(6):
            if x == 0:
                relative_attention_bias = True
            else:
                relative_attention_bias = False
            block = model.encoder.block[x]
            block.layer[0] = models.t5_hier.T5LayerSelfAttention(config, has_relative_attention_bias=relative_attention_bias)
            dblock = model.decoder.block[x]
            dblock.layer[0] = models.t5_hier.T5LayerSelfAttention(config, has_relative_attention_bias=relative_attention_bias)
            dblock.layer[1] = models.t5_hier.T5LayerCrossAttention(config)

        model.encoder.get_head_mask = partial(models.t5_hier.get_head_mask, [2,1,1,1,1], False)
        model.decoder.get_head_mask = partial(models.t5_hier.get_head_mask, [1,1,1,1,2], False)

        cp = "../../datasets/checkpoints_t5/checkpoint-2000/pytorch_model.bin"
        model.load_state_dict(torch.load(cp))

        train_batch_size = 6
        learning_rate = 1e-4

    training_args = TrainingArguments(output_dir="local",
                                      lr_scheduler_type="constant",
                                      learning_rate=learning_rate,
                                      warmup_steps=0.0,
                                      per_device_train_batch_size=train_batch_size,
                                      logging_steps=8,
                                      num_train_epochs=num_train_epochs,
                                      save_steps=1000,
                                      gradient_accumulation_steps=gradient_accumulation,
                                      weight_decay=.01
                                      )

    trainer = Trainer(
        model=model,
        args=training_args,
        train_dataset=tokenized_dataset,
        eval_dataset=tokenized_dataset,
    )
    trainer.train()


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
