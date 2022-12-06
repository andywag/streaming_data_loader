import os

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
import config_loader


""" Basic 
"""

def run_loader(args):
    if len(args.cache) == 0:
        subprocess.run(["cargo", "run", "--release", "--", "--task", args.task], cwd="../rust")
    else:
        print("Running with Cache", args.cache)
        subprocess.run(["cargo", "run", "--release", "--", "--task", args.task, '--cache', args.cache], cwd="../rust")



def run_model(args):
    # Get the Common Configuration
    tokenizer_name = "bert-base-uncased"
    sequence_length = 128
    batch_size = 32768
    if args.task == 'mlm':
        tokenizer_name = "bert-base-uncased"
    elif args.task == 'clm':
        tokenizer_name = "gpt2"
    elif args.task == 't5':
        tokenizer_name = "t5-small"

    fields = ["input_ids", "attention_mask", "labels"]
    if args.task == 'squad':
        fields = ["input_ids", "attention_mask", "token_type_ids", "start_positions", "end_positions"]
    elif args.task == 'single':
        fields = ["input_ids", "attention_mask", "token_type_ids", "label"]
    elif args.task == 'multi':
        fields = ["input_ids", "attention_mask", "token_type_ids", "labels"]

    def tokenize_function(examples):
        tokenizer = AutoTokenizer.from_pretrained(tokenizer_name)
        data = tokenizer(examples['text'], padding="max_length", truncation=True, max_length=sequence_length)
        return data




    learning_rate = 1e-5
    if args.task == 'mlm' or args.task == 'python':
        config = AutoConfig.from_pretrained(tokenizer_name)
        if args.small or args.task == 'python':
            config.num_hidden_layers = 12
            config.hidden_size = 768
            config.intermediate_size = 3072
            config.vocab_size = 4096
            config.num_attention_heads = 12
            learning_rate = 1e-5
        model = AutoModelForMaskedLM.from_config(config=config).train()
    elif args.task == 'clm':
        config = GPT2Config.from_pretrained(tokenizer_name)
        model = AutoModelForCausalLM.from_config(config=config).train()
    elif args.task == 'squad':
        config = AutoConfig.from_pretrained("bert-base-uncased")
        model = AutoModelForQuestionAnswering.from_pretrained("bert-base-uncased",config=config).train()
        sequence_length = 384
    elif args.task == 'single':
        config = AutoConfig.from_pretrained("bert-base-uncased")
        config.problem_type = "single_label_classification"
        config.num_labels = 2
        model = AutoModelForSequenceClassification.from_pretrained("bert-base-uncased",config=config).train()
    elif args.task == 'multi':
        config = AutoConfig.from_pretrained("bert-base-uncased")
        config.problem_type = "multi_label_classification"
        config.num_labels = 9
        model = AutoModelForSequenceClassification.from_pretrained("bert-base-uncased",config=config).train()
    elif args.task == 't5':
        config = AutoConfig.from_pretrained(tokenizer_name)
        model = T5ForConditionalGeneration.from_pretrained(tokenizer_name, config=config).train()



    tokenized_dataset = ExternalDataset("ipc:///tmp/masking_train",
                                        batch_size, fields=fields)

    training_args = TrainingArguments(output_dir="local",
                                      #lr_scheduler_type="constant",
                                      learning_rate=learning_rate,
                                      warmup_steps=0.0,
                                      per_device_train_batch_size=4,
                                      logging_steps=8,
                                      num_train_epochs=1,
                                      save_steps=1000000,
                                      gradient_accumulation_steps=8
                                      )

    trainer = Trainer(
        model=model,
        args=training_args,
        train_dataset=tokenized_dataset,
        eval_dataset=tokenized_dataset,
    )
    trainer.train()


parser = argparse.ArgumentParser(description='Run Model with External Data Loader')
parser.add_argument('--task', type=str, choices=["mlm", "clm", "t5", "squad", "single", "multi", "python"], default="python")
parser.add_argument('--config', type=str, default='git_python')
parser.add_argument('--all', action='store_true', default=True)
parser.add_argument('--small', action='store_true',default=True )
parser.add_argument('--cache', type=str, default='../../../storage')



def main():
    args = parser.parse_args()
    print(args)

    if args.all:
        pr = mp.Process(target=run_loader, args=(args,))
        pr.start()

    run_model(args)


if __name__ == '__main__':
    main()
