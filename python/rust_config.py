
class ExternalConfig:
    def __init__(self, config):
        self.config = config
        self.batch_size = self.config['batch']['batch_size']
        self.sequence_length = self.config['batch']['sequence_length']
        #self.tokenizer_name = self.config['tokenizer']['typ'].get('HuggingFace')


