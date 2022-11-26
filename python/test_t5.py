from transformers import T5Tokenizer

tokenizer = T5Tokenizer.from_pretrained("t5-small")
result = tokenizer.encode("I am here to save the day. The dog is done with the food.", add_special_tokens=True)
pass

#[27, 183, 270, 12, 1097, 8, 239, 5, 37, 1782, 19, 612, 28, 8, 542, 5, 1]