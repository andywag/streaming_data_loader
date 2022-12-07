from transformers.models.bert.modeling_bert import BertPreTrainedModel, BertEncoder
import torch.nn as nn
import torch
from typing import Optional, Tuple
import torch.nn.functional as F
from typing import Union

from transformers import BertLayer
from transformers.models.bert.modeling_bert import BaseModelOutputWithPastAndCrossAttentions
import math


def get_extended_attention_mask(attention_mask: torch.Tensor, input_shape: Tuple[int], device) -> torch.Tensor:

    return attention_mask


def expand_mask(attention_mask: torch.Tensor, input_shape: Tuple[int], device) -> torch.Tensor:
    # We can provide a self-attention mask of dimensions [batch_size, from_seq_length, to_seq_length]
    # ourselves in which case we just need to make it broadcastable to all heads.
    s = attention_mask.size()
    attn = attention_mask.reshape(shape=(s[0], s[1], s[2], 1))
    attn_rep = attn * torch.ones(size=(1, 512), device=device)
    attn_t = torch.transpose(attn_rep, 2, 3)
    mask_ind = attn_rep == attn_t
    extended_attention_mask = -1024.0 * torch.ones(size=mask_ind.size(), dtype=torch.float32, device=device)
    extended_attention_mask[mask_ind] = 0.0
    return extended_attention_mask


class BertLocalEncoder(nn.Module):
    def __init__(self, config):
        super().__init__()
        self.config = config
        self.layer = nn.ModuleList([BertLayer(config) for _ in range(config.num_hidden_layers)])
        self.l_embed =  nn.ModuleList([nn.Embedding(256, config.hidden_size) for _ in range(config.num_hidden_layers)])
        self.l_norm =  nn.ModuleList([nn.LayerNorm(config.hidden_size, eps=config.layer_norm_eps) for _ in range(config.num_hidden_layers)])

        self.gradient_checkpointing = False
        #for x in range(config.num_hidden_layers):
        #    self.layer[x].self = BertLocalSelfAttention(config)
        pass
    def forward(
        self,
        hidden_states: torch.Tensor,
        attention_mask: Optional[torch.FloatTensor] = None,
        head_mask: Optional[torch.FloatTensor] = None,
        encoder_hidden_states: Optional[torch.FloatTensor] = None,
        encoder_attention_mask: Optional[torch.FloatTensor] = None,
        past_key_values: Optional[Tuple[Tuple[torch.FloatTensor]]] = None,
        use_cache: Optional[bool] = None,
        output_attentions: Optional[bool] = False,
        output_hidden_states: Optional[bool] = False,
        return_dict: Optional[bool] = True,
    ) -> Union[Tuple[torch.Tensor], BaseModelOutputWithPastAndCrossAttentions]:
        all_hidden_states = () if output_hidden_states else None
        all_self_attentions = () if output_attentions else None
        all_cross_attentions = () if output_attentions and self.config.add_cross_attention else None

        next_decoder_cache = () if use_cache else None

        attention_mask_old = attention_mask
        attention_mask = expand_mask(attention_mask, None, attention_mask.device)

        for i, layer_module in enumerate(self.layer):
            if i < 3:
                mask = attention_mask[:,0,:,:]
            elif i < 6:
                mask = attention_mask[:,1,:,:]
            elif i < 9:
                mask = attention_mask[:,2,:,:]
            else:
                mask = attention_mask[:,3,:,:]
            mask = mask[:,None,:,:]

            if output_hidden_states:
                all_hidden_states = all_hidden_states + (hidden_states,)

            layer_head_mask = head_mask[i] if head_mask is not None else None
            past_key_value = past_key_values[i] if past_key_values is not None else None

            layer_outputs = layer_module(
                hidden_states,
                mask,
                layer_head_mask,
                encoder_hidden_states,
                encoder_attention_mask,
                past_key_value,
                output_attentions,
            )
            hidden_states = layer_outputs[0]
            if True:
                if i == 2:
                    hidden_states += self.l_embed[0](attention_mask_old[:,0,:])
                    hidden_states = self.l_norm[0](hidden_states)
                elif i == 5:
                    hidden_states += self.l_embed[1](attention_mask_old[:,1,:])
                    hidden_states = self.l_norm[1](hidden_states)
                elif i == 8:
                    hidden_states += self.l_embed[2](attention_mask_old[:,2,:])
                    hidden_states = self.l_norm[2](hidden_states)
                elif i == 11:
                    hidden_states += self.l_embed[3](attention_mask_old[:,3,:])
                    hidden_states = self.l_norm[3](hidden_states)

            #hidden_states = layer_outputs[0]
            if use_cache:
                next_decoder_cache += (layer_outputs[-1],)
            if output_attentions:
                all_self_attentions = all_self_attentions + (layer_outputs[1],)
                if self.config.add_cross_attention:
                    all_cross_attentions = all_cross_attentions + (layer_outputs[2],)

        if output_hidden_states:
            all_hidden_states = all_hidden_states + (hidden_states,)

        if not return_dict:
            return tuple(
                v
                for v in [
                    hidden_states,
                    next_decoder_cache,
                    all_hidden_states,
                    all_self_attentions,
                    all_cross_attentions,
                ]
                if v is not None
            )
        return BaseModelOutputWithPastAndCrossAttentions(
            last_hidden_state=hidden_states,
            past_key_values=next_decoder_cache,
            hidden_states=all_hidden_states,
            attentions=all_self_attentions,
            cross_attentions=all_cross_attentions,
        )

