<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

# 07 微调：SFT、LoRA 与 QLoRA

> 一句话定位：预训练让模型"会说话"，微调让模型"听话"；LoRA 用两个小矩阵把"听话"的成本从几百 GB 显存压到单张消费级显卡能跑。

## 直觉

预训练出来的 GPT 是一个"疯狂的续写机器"：你给它 "法国的首都是"，它可能续出 "法国的首都是巴黎，而意大利的首都是……" 一直写下去，因为它学的目标只是"预测下一个字"。它并没有学会"回答问题就该停下来"。

**指令微调（SFT, Supervised Fine-Tuning）** 做的事很朴素：拿一堆"人类问 + 人类满意的回答"配对，继续用同样的"预测下一个 token"目标训练，但**只在回答部分计算损失**。模型于是学会：看到问题格式，就输出回答格式。

问题来了——全参数微调一个 7B 模型，光是 AdamW 优化器状态（动量 + 二阶矩）就要存两份 fp32 副本，加上参数本身和梯度，显存需求是参数量的十几倍。7B 模型轻松吃掉 80GB+。个人根本玩不起。

**LoRA（Low-Rank Adaptation）** 的洞察是：微调对权重的改动 ΔW 其实"信息量很低"，可以用两个瘦长矩阵的乘积 B·A 来近似。原始权重 W 冻结不动（不需要优化器状态），只训练 A 和 B 这两个加起来可能只有原参数 0.1%~1% 的小矩阵。显存和存储瞬间降下来。

**QLoRA** 更进一步：连冻结的 W 都用 4bit 存（NF4 量化），显存再砍一大截，让单张 24GB 卡微调 33B 成为可能。

## 原理

### SFT 的 loss mask

一条 SFT 样本在拼接后形如：

```
[prompt tokens] [response tokens]
```

我们希望模型学"给定 prompt 生成 response"，而**不希望**它去学"生成 prompt"本身（prompt 是用户给的，模型不需要会编问题）。所以把 prompt 部分的 label 设成一个"忽略值"，交叉熵不在这些位置回传梯度。

本项目的 GPT 前向用的就是带忽略值的交叉熵（`torchgpt/model.py`）：

```python
# torchgpt/model.py  GPT.forward
loss = F.cross_entropy(
    logits.view(-1, logits.size(-1)), targets.view(-1), ignore_index=-1
)
```

注意这里 `ignore_index=-1`：凡是 label 为 `-1` 的位置都不计入损失。做 SFT 时，你只要把 prompt 段对应的 target 全部填成 `-1`，response 段填真实的下一个 token id，就实现了"只在 response 上算 loss"的 loss mask。

> 顺带一提：`torchgpt/dpo.py` 里的 `sequence_logprob` 用的忽略值是 `-100`（PyTorch 交叉熵的默认忽略值）。两处常数不同是因为它们是独立实现——用的时候按各自约定填 label 即可。

### 全参微调的显存账

对参数量为 N 的模型做 AdamW 全参微调，显存大致是：

| 项目 | 显存（以 fp32 计） |
|---|---|
| 参数 W | 4N |
| 梯度 ∇W | 4N |
| AdamW 一阶动量 m | 4N |
| AdamW 二阶矩 v | 4N |

合计约 **16N 字节**，还没算激活值。N=7B 时就是 ~112GB。这就是个人微调的拦路虎。

### LoRA 的低秩分解

设原始线性层权重为 W ∈ ℝ^(out×in)。LoRA 不直接改 W，而是在它旁边并联一条低秩旁路：

$$
h = Wx + \Delta W x,\qquad \Delta W = \frac{\alpha}{r}\, B A
$$

其中 A ∈ ℝ^(r×in)、B ∈ ℝ^(out×r)，秩 r ≪ min(in, out)。可训练参数从 `in×out` 降到 `r×(in+out)`。例如 in=out=4096、r=8 时，从约 1678 万降到约 6.6 万，缩小 250 倍。

**为什么 B 初始化为 0？** 这是 LoRA 的关键细节。训练刚开始时 ΔW = B·A = 0·A = 0，于是 h = Wx，**整个模型的输出和微调前一模一样**。这保证了训练起点是"原模型"，不会因为随机初始化的旁路一上来就扰乱已经预训练好的模型，训练更稳。A 用小随机数（而非也置零）是为了打破对称性，让 B 一旦开始更新就有非零梯度可循。

**α（alpha）与 r 的作用。** 缩放系数 α/r 控制旁路的"音量"。固定 α 时，改 r 不会同时改变有效学习幅度，调参更解耦。常见做法是 α 取 r 的 1~2 倍（本项目默认 r=8、α=16，即缩放 2.0）。r 越大，ΔW 的表达能力越强、能拟合更复杂的改动，但参数更多、也更容易过拟合小数据集。

## 结合本项目代码

LoRA 的全部实现就在 `torchgpt/lora.py`，短到可以逐行读。核心类 `LoRALinear`：

```python
# torchgpt/lora.py
class LoRALinear(nn.Module):
    def __init__(self, base: nn.Linear, r: int = 8, alpha: int = 16, dropout: float = 0.0):
        super().__init__()
        self.base = base
        for p in self.base.parameters():
            p.requires_grad = False  # 冻结原始权重
        self.r = r
        self.scaling = alpha / r
        self.lora_a = nn.Parameter(torch.randn(r, base.in_features) * 0.01)
        self.lora_b = nn.Parameter(torch.zeros(base.out_features, r))  # 初始为 0，训练起点等于原模型
        self.dropout = nn.Dropout(dropout)

    def forward(self, x):
        delta = (self.dropout(x) @ self.lora_a.t()) @ self.lora_b.t()
        return self.base(x) + delta * self.scaling
```

逐点对照前面的原理：

- `self.base` 的所有参数 `requires_grad = False`——这就是"冻结 W、不占优化器状态"。
- `lora_a` 形状 `(r, in_features)`，用 `randn * 0.01` 做小随机初始化 → 对应公式里的 A。
- `lora_b` 形状 `(out_features, r)`，`torch.zeros` 初始化 → 对应公式里的 B，置零保证训练起点 = 原模型。
- `self.scaling = alpha / r` → 公式里的 α/r 缩放。
- 前向 `self.base(x) + delta * self.scaling` 就是 `Wx + (α/r)·BA·x`。注意代码里先 `x @ lora_a.t()`（得到 r 维中间量）再 `@ lora_b.t()`，这正是把 ΔW·x 拆成 B(Ax) 两步小矩阵乘，省掉显式构造大矩阵 ΔW。

**往模型里安装 LoRA** 用 `apply_lora`，它把名字匹配的 `nn.Linear` 原地换成 `LoRALinear`：

```python
# torchgpt/lora.py
def apply_lora(model, r=8, alpha=16, targets=("c_attn", "c_proj")):
    for name, module in model.named_modules():
        for child_name, child in list(module.named_children()):
            if isinstance(child, nn.Linear) and child_name in targets:
                setattr(module, child_name, LoRALinear(child, r=r, alpha=alpha))
    return model
```

默认 `targets=("c_attn", "c_proj")` 正好对上 `torchgpt/model.py` 里注意力模块的两个线性层：`CausalSelfAttention` 的 `self.c_attn`（QKV 合并投影）和 `self.c_proj`（输出投影）。注意 `MLP` 里也有一个叫 `c_proj` 的层，按名字匹配它同样会被替换——这是"按名字匹配"的直接后果，心里要有数。

**冻结非 LoRA 参数** 用 `mark_only_lora_trainable`：

```python
# torchgpt/lora.py
def mark_only_lora_trainable(model):
    for name, p in model.named_parameters():
        p.requires_grad = "lora_" in name
```

它扫描所有参数名，只有名字里含 `"lora_"` 的（即 `lora_a` / `lora_b`）才 `requires_grad = True`，其余一律冻结。配合 `torchgpt/train.py` 里的 `configure_optimizer`——它只收集 `p.requires_grad` 为真的参数进优化器——优化器就只会为那几个小矩阵分配动量和二阶矩，显存账单因此大幅缩水。

**看看到底省了多少** 用 `trainable_parameter_ratio`：

```python
# torchgpt/lora.py
def trainable_parameter_ratio(model):
    trainable = sum(p.numel() for p in model.parameters() if p.requires_grad)
    total = sum(p.numel() for p in model.parameters())
    return trainable, total, trainable / total
```

对本项目默认配置（`GPTConfig`：n_embd=128、n_layer=4）跑一遍，你会看到可训练占比是个很小的小数。模型越大，这个比例越小——这正是 LoRA 的卖点。

### QLoRA（本项目未实现，仅讲原理）

QLoRA = **4bit 量化的冻结 base** + **LoRA 旁路**。思路：

1. 把 `base` 的权重用 **NF4（4-bit NormalFloat）** 量化存储。NF4 是针对"正态分布的权重"设计的信息论最优 4bit 数据类型，比普通 int4 更贴合权重实际分布。
2. 前向时按块把 4bit 权重**反量化**回 bf16 做矩阵乘，算完即丢，不常驻。
3. 只有 LoRA 的 A、B 以及它们的优化器状态是全精度、可训练的。
4. 再叠加 **双重量化**（把量化常数本身也量化）和 **分页优化器**（显存峰值溢出时把优化器状态挪到内存）进一步省显存。

在本项目的抽象层面，QLoRA 相当于把 `LoRALinear.base` 从一个 fp32 的 `nn.Linear` 换成一个"4bit 存、临时反量化"的线性层，而 `lora_a` / `lora_b` / `forward` 的结构完全不变。工程上一般用 `bitsandbytes` 的 `Linear4bit` 加 Hugging Face PEFT 实现，示意：

```python
# 伪代码，本项目未实现
import bitsandbytes as bnb
from peft import LoraConfig, get_peft_model
model = AutoModelForCausalLM.from_pretrained(name, load_in_4bit=True)  # NF4 量化 base
model = get_peft_model(model, LoraConfig(r=8, lora_alpha=16, target_modules=["c_attn", "c_proj"]))
```

## 常见坑

1. **忘记 `mark_only_lora_trainable` 或忘记冻结 base。** 只调 `apply_lora` 并不会自动冻结那些*没被替换*的层（如 embedding、LayerNorm、lm_head）。不显式冻结，它们仍会进优化器，省显存的目的就落空了。`LoRALinear` 内部只冻结了被它接管的那个 `base`。
2. **以为 `targets` 里的名字是全局唯一的。** `apply_lora` 按**子模块名**匹配，`model.py` 里注意力和 MLP 都有 `c_proj`，两处都会被换。想精确控制就得改匹配逻辑或改名。
3. **B 不置零、A 也置零。** 两个都置零 → ΔW 恒为 0 且 B 没有梯度，永远学不动；都用随机数 → 训练起点就偏离原模型，前几步 loss 可能飙高。本项目的"A 随机、B 置零"是经过验证的标准配方。
4. **α/r 理解反了。** 调大 r 若同时按比例调大 α，有效缩放 α/r 不变；想增强旁路影响力应单独调大 α。
5. **推理时忘了合并。** 部署时常把 ΔW=（α/r）BA 加回 W 得到单一权重（"merge"），省掉旁路分支的额外计算；本项目的 `LoRALinear.forward` 是不合并、始终并联两条路的训练态写法。

## 练习

1. **测量占比。** 写几行脚本：构造 `GPT(GPTConfig())`，调 `apply_lora(model)` 再 `mark_only_lora_trainable(model)`，最后 `print(trainable_parameter_ratio(model))`。记录可训练参数数、总数、比例。再把 `GPTConfig` 的 `n_embd` 从 128 改成 512 重跑，观察比例变大还是变小，并解释为什么。
2. **验证"训练起点 = 原模型"。** 对同一个 `GPT`，先记录 `model.generate(idx, 20, temperature=1.0)` 的输出（固定随机种子）；再 `apply_lora` 后、在**未做任何训练**的情况下重新生成。两次输出应完全一致。然后手动把某个 `LoRALinear.lora_b` 用 `torch.randn_like` 填上非零值，再生成一次，观察输出变化。用这个实验说明 B 置零的意义。
3. **实现 SFT loss mask。** 参照 `torchgpt/train.py` 的 `get_batch`，写一个 `get_sft_batch`：输入是 `(prompt_ids, response_ids)` 列表，拼成 x，并构造 y 使得 prompt 段对应位置为 `-1`（对齐 `model.py` 里 `cross_entropy` 的 `ignore_index=-1`）、response 段为真实的下一个 token。跑一个 batch 确认只有 response 段贡献 loss（提示：把 response 段也设成 -1，loss 应变成 0 或 NaN，以此反证）。

## 延伸阅读

- Hu et al., *LoRA: Low-Rank Adaptation of Large Language Models* (2021)
- Dettmers et al., *QLoRA: Efficient Finetuning of Quantized LLMs* (2023)
- Dettmers et al., *8-bit Optimizers via Block-wise Quantization* (2022)
- Ouyang et al., *Training language models to follow instructions with human feedback (InstructGPT)* (2022) —— SFT 阶段的经典出处
