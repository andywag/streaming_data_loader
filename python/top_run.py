
from transformers import AutoTokenizer, AutoModelForCausalLM, Trainer, TrainingArguments, GPT2Config, \
    AutoModelForMaskedLM, AutoModelForQuestionAnswering, AutoModelForSequenceClassification
from transformers import AutoConfig
from external_dataset import ExternalDataset
import argparse
import subprocess
import os
import multiprocessing as mp
import config_loader

os.environ['WANDB_DISABLED'] = 'true'


def run_loader(args):
    subprocess.run(["cargo", "run", "--release", "run", "zmq_none", args.file], cwd="../rust")


def run_model(input_config):
    # Get the Common Configuration
    tokenizer_name = input_config['tokenizer']['config']['tokenizer_name']
    sequence_length = input_config['tokenizer']['config']['sequence_length']
    batch_size = input_config['tokenizer']['config']['batch_size']

    def tokenize_function(examples):
        tokenizer = AutoTokenizer.from_pretrained(tokenizer_name)
        data = tokenizer(examples['text'], padding="max_length", truncation=True, max_length=sequence_length)
        return data

    fields = ["input_ids", "attention_mask", "labels"]
    if input_config['model'] == 'squad':
        fields = ["input_ids", "attention_mask", "token_type_ids", "start_positions", "end_positions"]
    elif input_config['model'] == 'single-class':
        fields = ["input_ids", "attention_mask", "token_type_ids", "label"]
    elif input_config['model'] == 'multi-label':
        fields = ["input_ids", "attention_mask", "token_type_ids", "labels"]

    tokenized_dataset = ExternalDataset(input_config['transport']['transport'].address,
                                        batch_size, fields=fields)

    if input_config['model'] == 'masking':
        config = AutoConfig.from_pretrained(tokenizer_name)
        model = AutoModelForMaskedLM.from_config(config=config).train()
    elif input_config['model'] == 'causal':
        config = GPT2Config.from_pretrained(tokenizer_name)
        model = AutoModelForCausalLM.from_config(config=config).train()
    elif input_config['model'] == 'squad':
        config = AutoConfig.from_pretrained("bert-base-uncased")
        model = AutoModelForQuestionAnswering.from_pretrained("bert-base-uncased",config=config).train()
    elif input_config['model'] == 'single-class':
        config = AutoConfig.from_pretrained("bert-base-uncased")
        config.problem_type = "single_label_classification"
        config.num_labels = 2
        model = AutoModelForSequenceClassification.from_pretrained("bert-base-uncased",config=config).train()
    elif input_config['model'] == 'multi-label':
        config = AutoConfig.from_pretrained("bert-base-uncased")
        config.problem_type = "multi_label_classification"
        config.num_labels = 9
        model = AutoModelForSequenceClassification.from_pretrained("bert-base-uncased",config=config).train()

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

    trainer = Trainer(
        model=model,
        args=training_args,
        train_dataset=tokenized_dataset,
        eval_dataset=tokenized_dataset,
    )
    trainer.train()


parser = argparse.ArgumentParser(description='Run Model with External Data Loader')
#parser.add_argument('--file', type=str, default="../rust/tests/gpt.yaml")
#parser.add_argument('--config', type=str, default="zmq_none")
parser.add_argument('--task', type=str, choices=["gpt2", "mlm", "squad", "imdb", "emot"], default="emot")
parser.add_argument('--all', action='store_true')


def main():
    args = parser.parse_args()

    config = "zmq_none"
    if args.task == 'gpt2':
        args.file = "../rust/tests/gpt.yaml"
    elif args.task == 'mlm':
        args.file = "../rust/tests/masking.yaml"
    elif args.task == 'squad':
        args.file = "../rust/tests/squad.yaml"
    elif args.task == 'imdb':
        args.file = "../rust/tests/single_class.yaml"
    elif args.task == 'emot':
        args.file = "../rust/tests/multi_label.yaml"

    if args.all:
        pr = mp.Process(target=run_loader, args=(args,))
        pr.start()

    config_file = config_loader.load(args.file)
    config = config_file[config]
    run_model(config)


if __name__ == '__main__':
    main()
