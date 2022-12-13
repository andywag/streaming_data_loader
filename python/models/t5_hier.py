import copy

import torch
from transformers import T5PreTrainedModel
from transformers.modeling_outputs import BaseModelOutputWithPastAndCrossAttentions
from transformers.models.t5.modeling_t5 import T5Block, T5LayerNorm, T5Attention


def expand_mask(attention_mask: torch.Tensor, device='cuda') -> torch.Tensor:
    # We can provide a self-attention mask of dimensions [batch_size, from_seq_length, to_seq_length]
    # ourselves in which case we just need to make it broadcastable to all heads.
    s = attention_mask.size()
    attn = attention_mask.reshape(shape=(s[0], s[1], s[2], 1))
    attn_rep = attn * torch.ones(size=(1, s[2]), device=device)
    attn_t = torch.transpose(attn_rep, 2, 3)
    mask_ind = attn_rep == attn_t
    extended_attention_mask = -10000.0 * torch.ones(size=mask_ind.size(), dtype=torch.float32, device=device)
    extended_attention_mask[mask_ind] = 0.0
    return extended_attention_mask

def get_head_mask(levels, reverse, head_mask, number_layers):
    #for x in range(len(head_mask)):
    if head_mask is None:
        return [None]*number_layers
    mask = expand_mask(head_mask)
    data = []
    for x in range(len(levels)):
        for y in range(levels[x]):
            if reverse:
                data.insert(0,mask[:,x,:,:])
            else:
                data.append(mask[:, x, :, :])
    real_mask = torch.stack(data, dim=1)
    s = real_mask.size()
    real_mask = real_mask.reshape(shape=(s[1], s[0], s[2], s[3]))
    return real_mask



class T5LayerSelfAttention(torch.nn.Module):
    def __init__(self, config, has_relative_attention_bias=False):
        super().__init__()
        self.SelfAttention = T5Attention(config, has_relative_attention_bias=has_relative_attention_bias)
        self.layer_norm = T5LayerNorm(config.d_model, eps=config.layer_norm_epsilon)
        self.dropout = torch.nn.Dropout(config.dropout_rate)

    def forward(
        self,
        hidden_states,
        attention_mask=None,
        position_bias=None,
        layer_head_mask=None,
        past_key_value=None,
        use_cache=False,
        output_attentions=False,
    ):
        #attention_mask = attention_mask + layer_head_mask
        s = layer_head_mask.size()
        layer_head_mask = layer_head_mask.reshape(shape=(s[0],1,s[1],s[2]))
        attention_mask = layer_head_mask
        layer_head_mask = None
        normed_hidden_states = self.layer_norm(hidden_states)
        attention_output = self.SelfAttention(
            normed_hidden_states,
            mask=attention_mask,
            position_bias=position_bias,
            layer_head_mask=layer_head_mask,
            past_key_value=past_key_value,
            use_cache=use_cache,
            output_attentions=output_attentions,
        )
        hidden_states = hidden_states + self.dropout(attention_output[0])
        outputs = (hidden_states,) + attention_output[1:]  # add attentions if we output them
        return outputs

class T5LayerCrossAttention(torch.nn.Module):
    def __init__(self, config):
        super().__init__()
        new_config = copy.deepcopy(config)
        new_config.is_decoder = True
        self.EncDecAttention = T5Attention(new_config, has_relative_attention_bias=False)
        self.layer_norm = T5LayerNorm(new_config.d_model, eps=config.layer_norm_epsilon)
        self.dropout = torch.nn.Dropout(new_config.dropout_rate)

    def forward(
        self,
        hidden_states,
        key_value_states,
        attention_mask=None,
        position_bias=None,
        layer_head_mask=None,
        past_key_value=None,
        use_cache=False,
        query_length=None,
        output_attentions=False,
    ):
        layer_head_mask = None
        normed_hidden_states = self.layer_norm(hidden_states)
        attention_output = self.EncDecAttention(
            normed_hidden_states,
            mask=attention_mask,
            key_value_states=key_value_states,
            position_bias=position_bias,
            layer_head_mask=layer_head_mask,
            past_key_value=past_key_value,
            use_cache=use_cache,
            query_length=query_length,
            output_attentions=output_attentions,
        )
        layer_output = hidden_states + self.dropout(attention_output[0])
        outputs = (layer_output,) + attention_output[1:]  # add attentions if we output them
        return outputs