# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""torchgpt.dpo —— Direct Preference Optimization（直接偏好优化）。

RLHF 的轻量替代：不训练奖励模型、不跑 PPO，直接用成对的「更好/更差」回答
优化策略模型。核心是让策略模型相对参考模型，拉大 chosen 与 rejected 的对数似然差：

    loss = -log σ( β · [ (logπ(y_w|x) - logπ(y_l|x)) - (logπ_ref(y_w|x) - logπ_ref(y_l|x)) ] )

其中 π 是待训练策略，π_ref 是冻结的参考模型（通常是 SFT 后的模型）。
详见 docs/08-对齐-RLHF-PPO-DPO.md。
"""

from __future__ import annotations

import torch
import torch.nn.functional as F


def sequence_logprob(model, input_ids, labels):
    """计算模型对 ``labels`` 序列的总对数似然（label 为 -100 的位置忽略）。"""
    logits, _ = model(input_ids)
    logits = logits[:, :-1, :]
    labels = labels[:, 1:]
    logp = F.log_softmax(logits, dim=-1)
    mask = labels != -100
    safe = labels.clamp_min(0).unsqueeze(-1)
    token_logp = torch.gather(logp, -1, safe).squeeze(-1)
    return (token_logp * mask).sum(dim=-1)


def dpo_loss(
    policy_chosen_logps,
    policy_rejected_logps,
    ref_chosen_logps,
    ref_rejected_logps,
    beta: float = 0.1,
):
    pi_logratios = policy_chosen_logps - policy_rejected_logps
    ref_logratios = ref_chosen_logps - ref_rejected_logps
    logits = pi_logratios - ref_logratios
    loss = -F.logsigmoid(beta * logits).mean()
    # 附带返回隐式奖励，便于监控 chosen/rejected 的分离程度
    chosen_reward = beta * (policy_chosen_logps - ref_chosen_logps).detach()
    rejected_reward = beta * (policy_rejected_logps - ref_rejected_logps).detach()
    return loss, chosen_reward, rejected_reward
