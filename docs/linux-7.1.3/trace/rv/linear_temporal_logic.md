## 线性时序逻辑（Linear temporal logic）


### 简介


运行时验证监视器（Runtime verification monitor）是一种验证技术，用于检查内核是否遵循某规格说明（specification）。它通过使用 tracepoint 监视内核的执行轨迹，并验证该执行轨迹满足规格说明来实现。

最初，规格说明只能以确定性自动机（DA）的形式编写。然而，在尝试为一些复杂规格说明实现 DA 监视器时，人们发现确定性自动机作为规格说明语言并不合适。该自动机复杂、难以理解且容易出错。

因此，引入了基于线性时序逻辑（LTL）的 RV 监视器。这类监视器使用 LTL 而非 DA 作为规格说明。在某些情况下，将规格说明写成 LTL 更为简洁和直观。
```

  Christel Baier and Joost-Pieter Katoen: Principles of Model Checking, The MIT
  Press, 2008.

```
### 语法（Grammar）


与某些现有语法不同，内核的 LTL 实现更为冗长。这是考虑到阅读 LTL 规格说明的人可能并不精通 LTL。

语法：
    ltl ::= opd | ( ltl ) | ltl binop ltl | unop ltl

操作数（opd）：
    true、false、由大写字母、数字和下划线组成的用户定义名称。

一元运算符（unop）：
    always（总是）
    eventually（最终）
    next（下一时刻）
    not（非）

二元运算符（binop）：
    until（直到）
    and（与）
    or（或）
    imply（蕴含）
    equivalent（等价）

该语法是歧义的：未定义运算符优先级。必须使用括号。

### 线性时序逻辑示例


   RAIN imply (GO_OUTSIDE imply HAVE_UMBRELLA)

含义：如果正在下雨，那么外出意味着带了伞。


   RAIN imply (WET until not RAIN)

含义：如果正在下雨，那么在下雨停止之前都会是湿的。


   RAIN imply eventually not RAIN

含义：如果正在下雨，雨最终会停。

上述示例仅指当前时间实例。对于内核验证，通常希望使用 `always` 运算符来指定
```

    always (RAIN imply eventually not RAIN)

```
含义：**所有**雨最终都会停。

在上述示例中，`RAIN`、`GO_OUTSIDE`、`HAVE_UMBRELLA` 和 `WET` 是“原子命题（atomic propositions）”。

### 监视器综合


要将 LTL 综合为内核监视器，可以使用 `rvgen` 工具：`tools/verification/rvgen`。规格说明需要以文件形式提供，
```

    RULE = always (ACQUIRE imply ((not KILLED and not CRASHED) until RELEASE))

```
其含义是：如果发生 `ACQUIRE`，则必须在 `KILLED` 或 `CRASHED` 之前发生 `RELEASE`。

可以使用子表达式将 LTL 拆分。上述等价于：

```

    RULE = always (ACQUIRE imply (ALIVE until RELEASE))
    ALIVE = not KILLED and not CRASHED

```
根据该规格说明，`rvgen` 会生成一个 Büchi 自动机的 C 实现——一个用于检查 LTL 可满足性的非确定性状态机。关于使用 `rvgen` 的细节，请参见 Documentation/trace/rv/monitor_synthesis.rst。

### 参考文献


```

  Christel Baier and Joost-Pieter Katoen: Principles of Model Checking, The MIT
  Press, 2008.

```
```

  Ruijie Meng, Zhen Dong, Jialin Li, Ivan Beschastnikh, and Abhik Roychoudhury.
  2022. Linear-time temporal logic guided greybox fuzzing. In Proceedings of the
  44th International Conference on Software Engineering (ICSE '22).  Association
  for Computing Machinery, New York, NY, USA, 1343–1355.
  https://doi.org/10.1145/3510003.3510082

```
```

  Gerth, R., Peled, D., Vardi, M.Y., Wolper, P. (1996). Simple On-the-fly
  Automatic Verification of Linear Temporal Logic. In: Dembiński, P., Średniawa,
  M. (eds) Protocol Specification, Testing and Verification XV. PSTV 1995. IFIP
  Advances in Information and Communication Technology. Springer, Boston, MA.
  https://doi.org/10.1007/978-0-387-34892-6_1

```
