<!-- Copyright (C) 2026 annsshadow -->
<!-- SPDX-License-Identifier: AGPL-3.0-or-later -->

# 04 · Transformer 结构：从一个 Block 到完整 GPT

> 一句话定位：GPT 是一摞结构完全相同的 decoder-only Transformer 块（Block），每块都在"自注意力 + 前馈网络"外面包了两条残差连接，靠堆叠深度把下一个 token 预测得越来越准。

本篇对照项目里两份真实实现来讲结构：纯 NumPy 的 `minigrad/nn.py` 和 PyTorch 的 `torchgpt/model.py`。两者结构**完全一致**，只是一个手写、一个调库，正好用来把"从零理解"和"工程实现"接起来。

---

## 1. 直觉：为什么是"嵌入 → 很多相同的块 → 输出头"

把一句话喂给模型，它内部发生的事情可以这样想象：

1. 每个 token 先变成一个向量（嵌入），这是它的"初始含义"。
2. 向量们排成一队穿过很多层相同的"加工车间"（Block）。每个车间做两件事：先让每个 token **环顾它左边的所有 token**，按相关性把信息搬过来（自注意力）；再让每个 token **各自独立地想一想**，把搬来的信息消化成新的表示（前馈网络 MLP）。
3. 车间越叠越多，token 的向量就从"字面含义"逐渐变成"结合了上下文的、足以预测下一个字的含义"。
4. 最后一层把每个位置的向量投影回词表大小，得到"下一个 token 是谁"的打分（logits）。

关键在于：**所有车间长得一模一样**（同样的层类型、同样的维度），只是参数不同。这种同构堆叠让模型可以任意加深，也让代码极其简洁——写一个 `Block`，然后 `for` 循环堆 N 份就行。

---

## 2. 整体结构：真实代码里的数据流

先看 `minigrad/nn.py` 里 `GPT.forward` 的主干（来源：`minigrad/nn.py`）：

```python
def forward(self, idx):
    """idx: 整型数组 (B, T)，返回 logits (B, T, vocab)。"""
    idx = np.asarray(idx)
    B, T = idx.shape
    assert T <= self.cfg.block_size, "序列超过 block_size"
    x = self.wte(idx) + self.wpe(np.arange(T))  # token + 位置嵌入
    for block in self.blocks:
        x = block(x)
    x = self.ln_f(x)
    return self.head(x)
```

五步，一一对应上面的直觉：

1. `self.wte(idx)`：token 嵌入，`(B,T) → (B,T,C)`，`C = n_embd`。
2. `+ self.wpe(np.arange(T))`：加上位置嵌入。注意力本身对位置是"无感"的（打乱顺序结果一样），所以必须显式注入位置信息。这里用的是**可学习的位置嵌入表** `wpe`（见 `GPT.__init__` 里 `self.wpe = Embedding(cfg.block_size, cfg.n_embd)`），而不是原始论文的正弦编码——这正是 GPT-2 的做法。
3. `for block in self.blocks`：穿过 `n_layer` 个相同的 `Block`。
4. `self.ln_f(x)`：终层 LayerNorm，输出头之前再归一化一次。
5. `self.head(x)`：投影到词表，得到 `(B,T,vocab)` 的 logits。

`torchgpt/model.py` 的 `GPT.forward` 一字不差地对应（来源：`torchgpt/model.py`）：

```python
pos = torch.arange(T, device=idx.device)
x = self.transformer.drop(self.transformer.wte(idx) + self.transformer.wpe(pos))
for block in self.transformer.h:
    x = block(x)
x = self.transformer.ln_f(x)
logits = self.lm_head(x)
```

区别只有两点：PyTorch 版多了一个 `drop`（dropout，`minigrad` 为教学清晰省去了），以及把子模块装进 `nn.ModuleDict`。结构骨架完全相同。

---

## 3. 残差连接：深网络能训起来的根本原因

看 `minigrad/nn.py` 里的 `Block.forward`（来源：`minigrad/nn.py`）：

```python
def forward(self, x):
    x = x + self.attn(self.ln1(x))
    x = x + self.mlp(self.ln2(x))
    return x
```

两行里的 `x +` 就是**残差连接**（residual / skip connection）。子层算出的不是"新的 x"，而是"要在旧 x 上做的修正量"，再加回去。

**为什么重要？** 从反向传播看最清楚。设某子层为 $F$，残差块是 $y = x + F(x)$。对 $x$ 求导：

$$\frac{\partial y}{\partial x} = I + \frac{\partial F}{\partial x}$$

那个 **$I$（单位矩阵）** 是关键。多层堆叠时，梯度按链式法则连乘：

$$\frac{\partial \mathcal{L}}{\partial x_0} = \prod_{l} \left(I + \frac{\partial F_l}{\partial x_l}\right)$$

展开后始终有一条"全是 $I$ 相乘"的通路，梯度可以**原封不动地流回浅层**，不会因为连乘许多小于 1 的数而指数衰减（梯度消失）。没有残差，几十层的 Transformer 几乎无法训练。

在 `minigrad` 里这件事是"免费"得到的：`x + self.attn(...)` 用的就是 `engine.Tensor.__add__`，它的 `_backward` 把 `out.grad` 原样分发给两个加数（来源：`minigrad/engine.py`）：

```python
def _backward():
    self.grad += self._unbroadcast(out.grad, self.data.shape)
    other.grad += self._unbroadcast(out.grad, other.data.shape)
```

`self`（残差主干）拿到的梯度就是 `out.grad` 本身——这正是上面公式里的那个 $I$。

---

## 4. Pre-LN vs Post-LN：归一化放在残差里面还是外面

LayerNorm 的位置有两种流派：

- **Post-LN**（原始 Transformer，2017）：`x = LN(x + F(x))`，归一化在残差相加**之后**。
- **Pre-LN**（GPT-2 及之后几乎所有大模型）：`x = x + F(LN(x))`，归一化在子层**之前**，残差主干上没有任何归一化挡着。

本项目两份实现都用 **Pre-LN**。看 `Block.forward` 里 `ln1`/`ln2` 都包在 `attn`/`mlp` 内部、`x +` 外部，残差主干 `x` 自己是"干净"的。

**为什么选 Pre-LN？** Post-LN 的残差通路上每层都压了一个 LayerNorm，梯度回流时被反复缩放，深层网络训练初期极不稳定，往往需要很长的 warmup 和很小的学习率才不发散。Pre-LN 让残差主干变成一条"无遮挡的高速公路"（上一节公式里的 $I$ 通路彻底畅通），梯度能干净地流到底，训练稳定得多——代价是最后需要补一个终层归一化 `ln_f`（第 2 节第 4 步），否则主干上累加的数值会越来越大。这就是为什么 `GPT` 里既有每块的 `ln1/ln2`，又在循环结束后单独来一个 `self.ln_f(x)`。

---

## 5. LayerNorm：原理与它的反向传播

LayerNorm 沿**特征维**（最后一维，长度 `n_embd`）把每个 token 的向量标准化到零均值、单位方差，再用可学习的 `gamma`（缩放）和 `beta`（平移）还原表达能力。公式：

$$\mu = \frac{1}{C}\sum_i x_i,\quad \sigma^2 = \frac{1}{C}\sum_i (x_i-\mu)^2,\quad y_i = \gamma_i \cdot \frac{x_i-\mu}{\sqrt{\sigma^2+\epsilon}} + \beta_i$$

它和 BatchNorm 的关键区别：**统计量只在单个样本的特征维上算**，与 batch 里的其他样本无关。因此它对 batch 大小不敏感，也天然适合变长序列和自回归推理（推理时 batch 甚至只有 1）——这是 Transformer 一律用 LayerNorm 而非 BatchNorm 的原因。

`minigrad` 把它完全用基础算子组合出来（来源：`minigrad/engine.py`）：

```python
def layernorm(x, gamma, beta, eps=1e-5):
    mu = x.mean(axis=-1, keepdims=True)
    xc = x - mu
    var = (xc * xc).mean(axis=-1, keepdims=True)
    std = (var + eps) ** 0.5
    return xc / std * gamma + beta
```

这里有个**教学上极其重要的点**：这个函数里**没有写一行反向传播代码**，但它的梯度完全正确。因为 `mean`、`-`、`*`、`** 0.5`、`/` 每一个都是 `engine` 里实现过 `_backward` 的基础算子，autograd 会沿计算图自动把它们的梯度串起来。LayerNorm 的手工反向推导非常繁琐（涉及 $\mu$ 和 $\sigma$ 对每个 $x_i$ 的交叉依赖），而这里"实现一次基础算子、复杂函数反向就免费"——这正是 autograd 的威力。`tests/test_engine.py::test_layernorm` 用数值梯度验证了它确实对。

`nn.LayerNorm`（`minigrad/nn.py`）只是把 `gamma=ones`、`beta=zeros` 两个参数包起来并转调这个函数；`torchgpt` 则直接用 `nn.LayerNorm(cfg.n_embd)`。

---

## 6. MLP / FFN：为什么要 4 倍扩张，为什么用 GELU

每个 Block 的后半部分是逐位置前馈网络（`minigrad/nn.py` 的 `MLP`）：

```python
class MLP(Module):
    """逐位置前馈网络，隐藏层扩张 4 倍（Transformer 标配）。"""
    def __init__(self, n_embd):
        self.fc = Linear(n_embd, 4 * n_embd)
        self.proj = Linear(4 * n_embd, n_embd)

    def forward(self, x):
        return self.proj(engine.gelu(self.fc(x)))
```

它做的是 `n_embd → 4·n_embd →（GELU）→ n_embd` 的"先放大再收回"。

**为什么是前馈网络？** 自注意力负责"token 之间搬运信息"（混合不同位置），但它本质是对 value 的加权平均，表达能力有限。MLP 负责"token 内部加工信息"：它对每个位置**独立**地做一次非线性变换，是 Transformer 里主要的非线性与记忆容量来源。注意力管"看哪里"，MLP 管"想什么"。

**为什么扩张 4 倍？** 中间升到 `4·n_embd` 给非线性变换足够的"工作空间"，让模型能表达更复杂的函数；4 倍是 GPT 系列沿用的经验值（不是定理，但被反复验证好用）。它也是参数量的大头（见第 8 节）。

**为什么用 GELU 而不是 ReLU？** GELU（Gaussian Error Linear Unit）可以理解为"平滑版 ReLU"：ReLU 在 0 处硬转折、负半轴梯度恒为 0；GELU 在 0 附近平滑过渡、负半轴有一小段非零梯度，优化更顺滑。`minigrad` 用的是 GPT-2/nanoGPT 同款的 **tanh 近似**（来源：`minigrad/engine.py`）：

```python
def gelu(x):
    c = np.sqrt(2.0 / np.pi)
    inner = (x + x ** 3 * 0.044715) * c
    return x * (inner.tanh() + 1.0) * 0.5
```

对应公式：

$$\text{GELU}(x) \approx 0.5\,x\left(1 + \tanh\!\left[\sqrt{\tfrac{2}{\pi}}\,(x + 0.044715\,x^3)\right]\right)$$

同样地，它由 `+`、`**`、`*`、`tanh` 组合而成，反向自动正确。`torchgpt` 直接用 `F.gelu`。

---

## 7. 因果自注意力：结构位置与 mask（结构速览）

自注意力的数学细节另有专篇，这里只从**结构**角度点明它在 Block 里的位置和两份实现的对应关系。`minigrad` 的 `CausalSelfAttention.forward`（来源：`minigrad/nn.py`）核心是：

```python
att = (q @ k.transpose((0, 1, 3, 2))) * (1.0 / np.sqrt(d))  # (B,H,T,T)
mask = np.triu(np.ones((T, T)), k=1) * -1e9  # 未来位置置 -inf
att = engine.softmax(att + Tensor(mask), axis=-1)
y = att @ v
```

- `(B,T,C)` 先经三个独立 `Linear` 得到 Q/K/V，`reshape + transpose` 切成 `n_head` 个头。
- `q @ kᵀ / sqrt(d)` 是缩放点积注意力分数，形状 `(B,H,T,T)`。
- 上三角 mask 把"看向未来"的分数置为 `-1e9`，softmax 后≈0，保证位置 i 只能看 ≤ i 的 token——这就是"因果 / causal"，GPT 自回归生成的根本前提。

`torchgpt` 把 Q/K/V 合并成一个 `c_attn = nn.Linear(n_embd, 3*n_embd)` 再 `split`（更省矩阵乘），并在 PyTorch 2 上优先走 `F.scaled_dot_product_attention(..., is_causal=True)`（FlashAttention 快路径），没有则回退到和 `minigrad` 一样的手写 masked softmax。两者数学等价，`torchgpt` 版更快更省显存。

---

## 8. 参数量估算：钱花在哪了

会估参数量，才能判断模型多大、显存够不够（下一步会在"规模化训练"里用到）。按 `minigrad` 的 `GPT` 逐块数（忽略 LayerNorm 的 `gamma/beta` 和 bias 这类小项，设 `C = n_embd`）：

**每个 Block：**
- 注意力：Q、K、V、proj 四个 `Linear(C, C)`，约 $4C^2$。
- MLP：`fc` 是 `C→4C`、`proj` 是 `4C→C`，约 $8C^2$。
- 合计每块约 $12C^2$。

**嵌入与头：**
- `wte`：`vocab_size × C`；`wpe`：`block_size × C`；`head`：`C × vocab_size`。

**全模型近似：**

$$N \approx 12\,C^2 \cdot n\_layer + (2\cdot\text{vocab} + \text{block\_size})\cdot C$$

对 `GPTConfig` 的默认 `n_layer=3, n_embd=64`（`minigrad`），主干约 $12\times64^2\times3 \approx 15$ 万参数，是个真正能在 CPU 上几十秒跑起来的玩具。真正的大模型就是把 `C` 和 `n_layer` 往上拉，参数量按 $C^2 \cdot L$ 大致平方级增长。记住这条经验公式：**Transformer 主干参数量 ≈ $12 \cdot n\_embd^2 \cdot n\_layer$**。

---

## 9. 权重共享：输入嵌入 = 输出投影

`torchgpt/model.py` 里有一行点睛之笔（来源：`torchgpt/model.py`）：

```python
# 权重共享：输入嵌入与输出投影同一张表（GPT-2 的做法，省参数、更稳）
self.transformer.wte.weight = self.lm_head.weight
```

输入端的 token 嵌入表 `wte`（`vocab × C`）和输出端的投影 `lm_head`（`C × vocab`，恰好形状转置）**共用同一块权重**。

**为什么合理？** 输入嵌入做的是"token id → 向量"，输出头做的是"向量 → token id 的打分"，二者是一件事的两个方向，用同一张表语义上自洽（这叫 weight tying）。好处有二：一是省掉一整张 `vocab × C` 的参数（词表大时这是实打实的大头）；二是经验上让训练更稳、泛化更好。

注意 `minigrad` 的 `GPT` **没有**做权重共享——它的 `wte` 和 `head` 是两套独立参数。这是两份实现刻意的差异：`minigrad` 重在把每一步讲清楚，`torchgpt` 向工程惯例对齐。读代码时留意这个区别，别以为 `minigrad` 漏了什么。

另外 `torchgpt` 的 `_init_weights` 用 `std=0.02` 的正态分布初始化所有 `Linear`/`Embedding` 权重（GPT-2 惯例）；`minigrad` 的 `Linear` 用 `1/sqrt(fan_in)` 缩放、`Embedding` 用 `std=0.02`——初始化方案不同但都遵循"小而有尺度"的原则。

---

## 10. 常见坑

1. **忘了位置嵌入**。只做 `wte(idx)` 不加 `wpe`，模型对词序完全无感，"狗咬人"和"人咬狗"没区别。本项目在 `forward` 第一行就 `wte(idx) + wpe(arange(T))`，别漏掉。
2. **Pre-LN 却忘了终层 `ln_f`**。Pre-LN 的残差主干会越加越大，不在输出前补一次归一化，logits 数值会漂、训练不稳。`GPT` 里的 `self.ln_f` 不是可有可无的装饰。
3. **LayerNorm 归一化错了维度**。必须沿最后一维（特征维）`axis=-1`，不是沿 batch 或时间维。`engine.layernorm` 写死了 `axis=-1`，自己改实现时极易弄错。
4. **残差写成了覆盖而非相加**。写成 `x = self.attn(self.ln1(x))`（丢了 `x +`）就退化成没有残差的普通堆叠，深了就训不动。
5. **序列超过 `block_size`**。位置嵌入表只有 `block_size` 行，`forward` 里的 `assert T <= block_size` 就是防线。推理时超长要像 `torchgpt` 的 `generate` 那样 `idx[:, -block_size:]` 截断。
6. **以为 `minigrad` 和 `torchgpt` 结构不同**。它们结构相同，差异只在：权重共享（仅 torchgpt）、dropout（仅 torchgpt）、Q/K/V 合并与 FlashAttention（仅 torchgpt）、以及初始化细节。

---

## 11. 练习（基于本项目代码）

1. **数参数**。给 `minigrad/nn.py` 的 `GPT` 写一小段：`sum(p.data.size for p in model.parameters())`，用默认 `GPTConfig` 跑出总参数量，再用第 8 节的公式 $12C^2L$ 手算主干部分，对比差多少、差在哪里（提示：嵌入、head、bias、LayerNorm 参数都不在 $12C^2L$ 里）。

2. **把 Pre-LN 改成 Post-LN**。复制 `Block.forward`，改成 `x = self.ln1(x + self.attn(x))` 的 Post-LN 形式，用 `minigrad/train.py` 的 `train()` 各跑一次，比较前几十步 loss 下降的稳定性。思考：为什么 Pre-LN 初期更稳？（可同时试着调大初始学习率放大差异。）

3. **验证权重共享**。在 `torchgpt/model.py` 的 `GPT` 构造完后打印 `model.transformer.wte.weight.data_ptr() == model.lm_head.weight.data_ptr()`，确认二者确实是同一块内存；再把那行共享删掉，用第 8 节方法数参数量，看权重共享到底省了多少（约等于 `vocab_size × n_embd`）。

---

## 12. 延伸阅读

- Vaswani et al., *Attention Is All You Need* (2017) —— Transformer 原始论文，Post-LN 版本。
- Radford et al., *Language Models are Unsupervised Multitask Learners* (GPT-2, 2019) —— decoder-only、Pre-LN、可学习位置嵌入、权重共享的来源。
- Ba et al., *Layer Normalization* (2016) —— LayerNorm 原理。
- He et al., *Deep Residual Learning for Image Recognition* (ResNet, 2015) —— 残差连接的奠基工作。
- Xiong et al., *On Layer Normalization in the Transformer Architecture* (2020) —— 系统分析 Pre-LN vs Post-LN 的训练稳定性。
- Hendrycks & Gimpel, *Gaussian Error Linear Units (GELUs)* (2016) —— GELU 激活。


