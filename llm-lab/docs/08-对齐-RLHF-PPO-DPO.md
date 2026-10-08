<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

# 08 对齐：RLHF、PPO 与 DPO

> 一句话定位：SFT 教会模型"回答问题的格式"，对齐（alignment）教会模型"在众多合格回答里挑人类更喜欢的那一个"——DPO 则用一行损失函数绕过了 RLHF 那套笨重的奖励模型 + 强化学习管线。

## 直觉

SFT 之后的模型已经会好好回答问题了，但"会回答"和"回答得好"是两回事。同一个问题，模型能生成一万种语法正确的答案，其中有的礼貌、有的傲慢，有的翔实、有的敷衍。我们想让它偏向人类喜欢的那种。

难点在于："人类喜欢"这件事**写不出标准答案**。你没法给每个问题准备一个唯一正确的 response 去做监督学习。但人类**很擅长比较**：给两个回答 A、B，普通人能轻松说出"B 更好"。对齐就建立在这种**成对偏好**数据上。

**RLHF（基于人类反馈的强化学习）** 的经典三段式：先 SFT，再训一个"奖励模型"去模仿人类的打分，最后用强化学习（PPO）让语言模型最大化这个奖励。它能用，但管线复杂、要同时在显存里塞四个模型、训练极不稳定。

**DPO（Direct Preference Optimization）** 的洞察：既然最终目标是让模型偏好 chosen 回答、冷落 rejected 回答，那何必绕一大圈先训奖励模型？数学上可以证明，"训奖励模型 + PPO 最大化奖励"这整个过程，等价于一个**直接在偏好对上做的分类损失**。于是 DPO 把奖励模型显式地消掉了，只留一个像逻辑回归一样简单的损失函数。

## 原理

### RLHF 三阶段

1. **SFT**：如第 07 讲，得到一个会听话的基座 π_SFT。
2. **奖励模型（RM）**：在 π_SFT 上接一个标量输出头，用成对偏好数据训练，使得对 chosen 回答 y_w 打的分高于 rejected 回答 y_l。损失是 Bradley-Terry 模型：`-log σ(r(x, y_w) - r(x, y_l))`。
3. **PPO**：把语言模型当作一个策略 π_θ，它每吐一个 token 就是一个动作，奖励模型给整段回答打分当作回报。用 PPO 这个强化学习算法更新 π_θ 去最大化期望奖励，同时加一个 **KL 惩罚**项，约束 π_θ 不要离 π_SFT（参考模型）太远，防止"钻奖励模型空子"跑飞（reward hacking）。

### PPO 为什么复杂（本项目未实现，仅讲原理）

PPO 阶段同时需要**四个模型**驻留：

- 策略模型 π_θ（在训，要梯度）
- 参考模型 π_ref（冻结，算 KL）
- 奖励模型 RM（冻结，打分）
- 价值模型（critic，估计优势函数，要梯度）

还要在线采样（on-policy，每步得让当前模型现场生成）、估计优势、裁剪比率、调一堆敏感超参。显存和工程复杂度都很高，训练稳定性是出了名的难伺候。伪代码示意：

```python
# 伪代码，本项目未实现
for prompt in prompts:
    response = policy.generate(prompt)                 # 在线采样
    reward   = reward_model(prompt, response)          # RM 打分
    kl       = logprob(policy, response) - logprob(ref, response)
    advantage = reward - kl_coef * kl - value(prompt)  # 带 KL 惩罚与基线
    ppo_update(policy, value, advantage)               # 裁剪目标 + 梯度上升
```

### DPO：绕过奖励模型

DPO 的推导起点是 RLHF 带 KL 约束的最优解。可以解析地写出：在"最大化奖励 r、同时 KL 约束 π 靠近 π_ref"的目标下，最优策略满足

$$
r(x, y) = \beta \log \frac{\pi^{*}(y|x)}{\pi_{\text{ref}}(y|x)} + \beta \log Z(x)
$$

关键在于：这把**奖励 r 用策略和参考策略的对数比表示出来了**。把它代回奖励模型的 Bradley-Terry 偏好损失，配分函数 Z(x)（那个难算的归一化项）在 chosen 和 rejected 相减时**正好抵消**。最终得到只依赖策略和参考模型对数似然的损失：

$$
\mathcal{L}_{\text{DPO}} = -\log \sigma\!\Big( \beta \big[ (\log\pi_\theta(y_w|x) - \log\pi_\theta(y_l|x)) - (\log\pi_{\text{ref}}(y_w|x) - \log\pi_{\text{ref}}(y_l|x)) \big] \Big)
$$

直观读法：括号里是"策略拉开 chosen/rejected 的程度" 减去 "参考模型本来就有的差距"。DPO 要最大化这个差，即**让策略相对参考模型，更偏爱 chosen、更嫌弃 rejected**。减去参考模型那一项是为了只奖励"微调带来的改进"，而非模型本来就有的偏好。

### β 的作用与隐式奖励

- **β** 控制对参考模型的偏离强度，扮演 RLHF 里 KL 惩罚系数的角色。β 小 → 允许策略大幅偏离 π_ref，更激进地迎合偏好，但可能生成退化/重复文本；β 大 → 更保守地贴近 π_ref。常用 0.1~0.5。
- **隐式奖励**：DPO 没有显式奖励模型，但 `β·log(π_θ/π_ref)` 本身就是一个"隐式奖励"。监控 chosen 和 rejected 的隐式奖励差，能看出模型有没有在学会区分好坏。

### 参考模型的角色

π_ref 通常就是 SFT 后的模型，**全程冻结**。它提供两个作用：(1) 作为 KL 锚点，防止策略跑飞；(2) 在损失里抵消掉"模型本来就有的偏好基线"。DPO 只需两个模型（策略 + 参考），比 PPO 的四个少一半，这也是它受欢迎的工程原因。

## 结合本项目代码

DPO 的核心就两个函数，都在 `torchgpt/dpo.py`。

### 算序列对数似然：`sequence_logprob`

DPO 损失需要四个量：策略/参考模型分别对 chosen/rejected 的序列对数似然。`sequence_logprob` 负责算其中一个：

```python
# torchgpt/dpo.py
def sequence_logprob(model, input_ids, labels):
    """计算模型对 labels 序列的总对数似然（label 为 -100 的位置忽略）。"""
    logits, _ = model(input_ids)
    logits = logits[:, :-1, :]
    labels = labels[:, 1:]
    logp = F.log_softmax(logits, dim=-1)
    mask = labels != -100
    safe = labels.clamp_min(0).unsqueeze(-1)
    token_logp = torch.gather(logp, -1, safe).squeeze(-1)
    return (token_logp * mask).sum(dim=-1)
```

逐步拆解：

- `logits = logits[:, :-1, :]` 与 `labels = labels[:, 1:]`：**错位一位**。因为位置 t 的 logits 预测的是位置 t+1 的 token，所以要把 logits 去掉最后一位、labels 去掉第一位对齐。这正是自回归语言模型的标准 shift。
- `F.log_softmax(logits, dim=-1)`：把 logits 变成每个词的对数概率。
- `mask = labels != -100`：**忽略 -100 的位置**。做 DPO 时，你会把 prompt 段的 label 填成 `-100`，于是只有 response 段进入对数似然——我们只关心模型对回答部分的偏好，不关心它复述问题的概率。
- `safe = labels.clamp_min(0)`：把 -100 这类负数夹到 0，避免 `gather` 的索引越界（这些位置反正会被 mask 清零，填谁都无所谓）。
- `torch.gather(logp, -1, safe)`：从每个位置的全词表对数概率里，**挑出真实 token 那一个**的对数概率。
- `(token_logp * mask).sum(dim=-1)`：mask 掉忽略位，对剩下的逐 token 对数概率**求和**，得到整段 response 的总对数似然 log π(y|x)。

### 算损失：`dpo_loss`

```python
# torchgpt/dpo.py
def dpo_loss(policy_chosen_logps, policy_rejected_logps,
             ref_chosen_logps, ref_rejected_logps, beta: float = 0.1):
    pi_logratios = policy_chosen_logps - policy_rejected_logps
    ref_logratios = ref_chosen_logps - ref_rejected_logps
    logits = pi_logratios - ref_logratios
    loss = -F.logsigmoid(beta * logits).mean()
    # 附带返回隐式奖励，便于监控 chosen/rejected 的分离程度
    chosen_reward = beta * (policy_chosen_logps - ref_chosen_logps).detach()
    rejected_reward = beta * (policy_rejected_logps - ref_rejected_logps).detach()
    return loss, chosen_reward, rejected_reward
```

和上面的公式严丝合缝：

- `pi_logratios` = 策略对 chosen 减对 rejected 的对数似然差，对应公式 (logπθ(y_w) − logπθ(y_l))。
- `ref_logratios` = 参考模型的同一个差，对应 (logπ_ref(y_w) − logπ_ref(y_l))。
- `logits = pi_logratios - ref_logratios` = 两者相减，就是公式括号里的整体。
- `loss = -F.logsigmoid(beta * logits).mean()` = `-log σ(β·logits)`，正是 DPO 损失（`logsigmoid` 比先 `sigmoid` 再 `log` 数值更稳）。
- `chosen_reward` / `rejected_reward` = 前面讲的**隐式奖励** `β·log(π_θ/π_ref)`，`.detach()` 表示它们只用于监控、不回传梯度。训练时若看到 `chosen_reward` 稳步高于 `rejected_reward`，说明模型正在学会偏好。

### 一次完整的 DPO 前向（把两个函数串起来，示意）

```python
# 伪代码，组合本项目已有的 API
pc = sequence_logprob(policy, chosen_ids, chosen_labels)
pr = sequence_logprob(policy, rejected_ids, rejected_labels)
with torch.no_grad():                       # 参考模型冻结，不要梯度
    rc = sequence_logprob(ref, chosen_ids, chosen_labels)
    rr = sequence_logprob(ref, rejected_ids, rejected_labels)
loss, chosen_reward, rejected_reward = dpo_loss(pc, pr, rc, rr, beta=0.1)
loss.backward()
```

四次 `sequence_logprob` 调用正好对上 `dpo_loss` 的四个入参。参考模型那两次放在 `torch.no_grad()` 里——它冻结、不更新。

## 常见坑

1. **label 的忽略值用错。** `sequence_logprob` 认的是 **-100**，而 `torchgpt/model.py` 的 `cross_entropy` 用的是 **-1**。做 DPO 构造 labels 时务必填 -100，否则 prompt 段会被错误地计入对数似然。
2. **忘了错位。** 若直接用 `logits` 和 `labels` 不做 shift，算出的是"模型对当前位置自己"的概率，完全错位。`sequence_logprob` 已经替你做了 `[:, :-1]` / `[:, 1:]`。
3. **参考模型没冻结 / 忘了 no_grad。** π_ref 必须全程不更新。忘了 `torch.no_grad()` 不仅浪费显存，若误把它的参数也丢进优化器还会污染训练。
4. **β 调极端。** β 太小模型容易生成重复退化文本并偏离太远；β 太大则几乎学不动偏好。先从 0.1 起步。
5. **以为 DPO 不需要 SFT。** DPO 的参考模型通常就是 SFT 模型，偏好数据也假设回答已经"基本合格"。跳过 SFT 直接 DPO，效果通常很差。
6. **chosen 和 rejected 长度悬殊。** 对数似然是逐 token 求和，长回答天然总对数概率更低（更多负数相加）。长度严重失衡的偏好对会给损失引入长度偏置，实践中常需要长度归一化或在数据上控制。

## 练习

1. **手算一次损失。** 构造两个小张量当作 `policy_chosen_logps=[-2.0]`、`policy_rejected_logps=[-3.0]`、`ref_chosen_logps=[-2.5]`、`ref_rejected_logps=[-2.5]`，调 `dpo_loss(..., beta=0.1)`。先用公式手算 `-log σ(0.1·((−2)−(−3))−((−2.5)−(−2.5)))` 的值，再和函数返回的 `loss` 对照。然后把 `policy_chosen_logps` 调高到 `-1.0`，观察 loss 变小、`chosen_reward` 变大，解释为什么。
2. **验证忽略位。** 构造一条 `input_ids` 和两份 `labels`：一份把前 3 个位置设为 -100，另一份不设。分别调 `sequence_logprob`，比较两个返回值的差，确认被 -100 掩掉的位置确实没有计入总和。
3. **串出一个最小 DPO step。** 用 `GPT(GPTConfig())` 建一个策略模型和一个 `copy.deepcopy` 出来的参考模型，随手造一对 (chosen, rejected) 的 token 序列，按上文"完整前向"的示意跑一次 `loss.backward()`，并 `print` 出 `chosen_reward` 和 `rejected_reward`。连续跑几步同一对数据，观察 chosen 与 rejected 的隐式奖励是否逐渐拉开。

## 延伸阅读

- Ouyang et al., *Training language models to follow instructions with human feedback (InstructGPT)* (2022) —— RLHF 三阶段的奠基作
- Rafailov et al., *Direct Preference Optimization: Your Language Model is Secretly a Reward Model* (2023) —— DPO 原始论文
- Schulman et al., *Proximal Policy Optimization Algorithms (PPO)* (2017)
- Christiano et al., *Deep Reinforcement Learning from Human Preferences* (2017) —— 偏好学习的源头
- Bai et al., *Constitutional AI: Harmlessness from AI Feedback* (2022) —— RLAIF 方向
