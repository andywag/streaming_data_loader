from torch.nn import CrossEntropyLoss
from transformers.modeling_outputs import MaskedLMOutput
from transformers.models.bert.modeling_bert import BertPreTrainedModel, BertEncoder, BertOnlyMLMHead
import torch.nn as nn
import torch
from typing import Optional, Tuple
import torch.nn.functional as F
from typing import Union

from transformers import BertLayer, BertModel
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
        self.l_embed =  nn.ModuleList([nn.Embedding(256, config.hidden_size) for _ in range(4)])
        self.l_norm =  nn.ModuleList([nn.LayerNorm(config.hidden_size, eps=config.layer_norm_eps) for _ in range(5)])

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
            elif i < 8:
                mask = attention_mask[:,2,:,:]
            elif i < 10:
                mask = attention_mask[:,3,:,:]
            else:
                mask = attention_mask[:,4,:,:]
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
                elif i == 7:
                    hidden_states += self.l_embed[2](attention_mask_old[:,2,:])
                    hidden_states = self.l_norm[2](hidden_states)
                elif i == 9:
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

class BertForMaskedLM(BertPreTrainedModel):

    _keys_to_ignore_on_load_unexpected = [r"pooler"]
    _keys_to_ignore_on_load_missing = [r"position_ids", r"predictions.decoder.bias", r"cls.predictions.decoder.weight"]

    def __init__(self, config):
        super().__init__(config)
        self.bert = BertModel(config, add_pooling_layer=False)
        self.cls = BertOnlyMLMHead(config)

        # Initialize weights and apply final processing
        #self.post_init()

    def get_output_embeddings(self):
        return self.cls.predictions.decoder

    def set_output_embeddings(self, new_embeddings):
        self.cls.predictions.decoder = new_embeddings

    def forward(
        self,
        input_ids: Optional[torch.Tensor] = None,
        attention_mask: Optional[torch.Tensor] = None,
        token_type_ids: Optional[torch.Tensor] = None,
        position_ids: Optional[torch.Tensor] = None,
        head_mask: Optional[torch.Tensor] = None,
        inputs_embeds: Optional[torch.Tensor] = None,
        encoder_hidden_states: Optional[torch.Tensor] = None,
        encoder_attention_mask: Optional[torch.Tensor] = None,
        masked_lm_positions: Optional[torch.Tensor] = None,
        masked_lm_labels: Optional[torch.Tensor] = None,
        output_attentions: Optional[bool] = None,
        output_hidden_states: Optional[bool] = None,
        return_dict: Optional[bool] = None,
    ) -> Union[Tuple[torch.Tensor], MaskedLMOutput]:
        r"""
        labels (`torch.LongTensor` of shape `(batch_size, sequence_length)`, *optional*):
            Labels for computing the masked language modeling loss. Indices should be in `[-100, 0, ...,
            config.vocab_size]` (see `input_ids` docstring) Tokens with indices set to `-100` are ignored (masked), the
            loss is only computed for the tokens with labels in `[0, ..., config.vocab_size]`
        """

        return_dict = return_dict if return_dict is not None else self.config.use_return_dict

        outputs = self.bert(
            input_ids,
            attention_mask=attention_mask,
            token_type_ids=token_type_ids,
            position_ids=position_ids,
            head_mask=head_mask,
            inputs_embeds=inputs_embeds,
            encoder_hidden_states=encoder_hidden_states,
            encoder_attention_mask=encoder_attention_mask,
            output_attentions=output_attentions,
            output_hidden_states=output_hidden_states,
            return_dict=return_dict,
        )

        sequence_output = outputs[0]
        prediction_scores = self.cls(sequence_output)

        masked_lm_loss = None
        if masked_lm_labels is not None:
            prediction_scores1 = prediction_scores[masked_lm_positions,:]
            loss_fct = CrossEntropyLoss()  # -100 index = padding token
            masked_lm_loss = loss_fct(prediction_scores.view(-1, self.config.vocab_size), masked_lm_labels.view(-1))

        if not return_dict:
            output = (prediction_scores,) + outputs[2:]
            return ((masked_lm_loss,) + output) if masked_lm_loss is not None else output

        return MaskedLMOutput(
            loss=masked_lm_loss,
            logits=prediction_scores,
            hidden_states=outputs.hidden_states,
            attentions=outputs.attentions,
        )

    def prepare_inputs_for_generation(self, input_ids, attention_mask=None, **model_kwargs):
        input_shape = input_ids.shape
        effective_batch_size = input_shape[0]

        #  add a dummy token
        if self.config.pad_token_id is None:
            raise ValueError("The PAD token should be defined for generation")

        attention_mask = torch.cat([attention_mask, attention_mask.new_zeros((attention_mask.shape[0], 1))], dim=-1)
        dummy_token = torch.full(
            (effective_batch_size, 1), self.config.pad_token_id, dtype=torch.long, device=input_ids.device
        )
        input_ids = torch.cat([input_ids, dummy_token], dim=1)

        return {"input_ids": input_ids, "attention_mask": attention_mask}


