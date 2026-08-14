

Researcher Guidelines
+++++++++++++++++++++

The Linux 内核 community welcomes transparent research 在 the Linux
内核, the activities involved 在 producing 它, 和 任何 其他 byproducts
的 其 development. Linux benefits greatly 来自 此 kind 的 research, 和
大多数 aspects 的 Linux 是 driven 由 research 在 one form 或 another.

The community greatly appreciates 若 researchers 可 share preliminary
findings 之前 making 它们的 results 公共, especially 若 此类 research
involves 安全. Getting involved early helps 两者 improve the quality
的 research 和 ability 用于 Linux 到 improve 来自 它. 在 任何 case,
sharing 打开 access copies 的 the published research 与 the community
是 recommended.

此 document seeks 到 clarify 什么 the Linux 内核 community considers
acceptable 和 non-acceptable practices 当 conducting 此类 research. 在
the very least, 此类 research 和 related activities 应当 follow
标准 research ethics rules. 用于 更多 background 在 research ethics
generally, ethics 在 technology, 和 research 的 developer communities
特别是, 参见:

- `History of Research Ethics <https://www.unlv.edu/research/ORI-HSR/history-ethics>`_
- `IEEE Ethics <https://www.ieee.org/about/ethics/index.html>`_
- `Developer and Researcher Views on the Ethics of Experiments on Open-Source Projects <https://arxiv.org/pdf/2112.13217.pdf>`_

The Linux 内核 community expects 该 everyone interacting 与 the
project 是 participating 在 good faith 到 make Linux better. Research 在
任何 publicly-available artifact (including, 但 不 limited 到 source
code) produced 由 the Linux 内核 community 是 welcome, though research
在 developers 必须 为 distinctly opt-in.

Passive research 即 based entirely 在 publicly 可用 sources,
including posts 到 公共 mailing 列表 和 commits 到 公共
repositories, 是 clearly permissible. Though, 作为 与 任何 research,
标准 ethics 必须 仍然 为 followed.

Active research 在 developer behavior, 然而, 必须 为 已完成 与 the
explicit agreement 的, 和 full disclosure 到, the 各个 developers
involved. Developers cannot 为 interacted 与/experimented 在 无
consent; 此, too, 是 标准 research ethics.

## Surveys


Research 通常 takes the form 的 surveys sent 到 maintainers 或
contributors.  作为 一个 通用 rule, though, the 内核 community derives
little 值 来自 这些 surveys.  The 内核 development 进程 works
因为 every developer benefits 来自 它们的 participation, even working
与 others 谁 具有 不同 goals.  Responding 到 一个 survey, though, 是 一个
one-way demand placed 在 busy developers 与 无 corresponding benefit 到
themselves 或 到 the 内核 community 作为 一个 whole.  用于 此 reason, 此
方法 的 research 是 discouraged.

内核 community members 已经 receive far too much email 和 是 likely
到 perceive survey requests 作为 just another demand 在 它们的 time.  Sending
此类 requests deprives the community 的 valuable contributor time 和 是
unlikely 到 yield 一个 statistically useful 响应.

作为 一个 alternative, researchers 应当 consider attending developer 事件,
hosting sessions 何处 the research project 和 其 benefits 到 the
participants 可 为 explained, 和 interacting directly 与 the community
那里.  The information received 将 为 far richer 比 该 obtained 来自
一个 email survey, 和 the community 将 gain 来自 the ability 到 learn 来自
您的 insights 作为 well.

## Patches


到 help clarify: sending patches 到 developers **是** interacting
与 them, 但 它们 具有 已经 consented 到 receiving *good faith
contributions*. Sending intentionally flawed/vulnerable patches 或
contributing misleading information 到 discussions 是 不 consented
到. 此类 communication 可 为 damaging 到 the developer (e.g. draining
time, effort, 和 morale) 和 damaging 到 the project 由 eroding
the entire developer community's trust 在 the contributor (和 the
contributor's organization 作为 一个 whole), undermining efforts 到 提供
constructive feedback 到 contributors, 和 putting end users 在 risk 的
软件 flaws.

Participation 在 the development 的 Linux itself 由 researchers, 作为
与 anyone, 是 welcomed 和 encouraged. Research 进入 Linux code 是
一个 通用 practice, especially 当 它 comes 到 developing 或 运行中
analysis tools 该 produce actionable results.

当 engaging 与 the developer community, sending 一个 patch 具有
traditionally 已经 the best way 到 make 一个 impact. Linux 已经 具有
plenty 的 known bugs -- 什么's much 更多 helpful 是 having vetted fixes.
之前 contributing, carefully 读取 the appropriate documentation:

- Documentation/进程/development-process.rst
- Documentation/进程/submitting-patches.rst
- Documentation/admin-guide/reporting-issues.rst
- Documentation/进程/security-bugs.rst

然后 send 一个 patch (including 一个 commit log 与 全部 the details listed
下文) 和 follow up 在 任何 feedback 来自 其他 developers.

当 sending patches produced 来自 research, the commit logs 应当
包含 至少 the 以下 details, 因此 该 developers 具有
appropriate 上下文 用于 understanding the contribution. Answer:

- 什么 是 the 特定 problem 该 具有 已经 found?
- 如何 可以 the problem 为 reached 在 一个 运行中 系统?
- 什么 effect 将会 encountering the problem 具有 在 the 系统?
- 如何 曾是 the problem found? Specifically 包含 details 关于 任何
  testing, 静态 或 动态 analysis programs, 和 任何 其他 tools 或
  方法 使用 到 perform the work.
- 其 版本 的 Linux 曾是 the problem found 在? 使用 the 大多数 recent
  释放 或 一个 recent linux-next branch 是 strongly preferred (参见
  Documentation/进程/howto.rst).
- 什么 曾是 changed 到 fix the problem, 和 为何 它是 believed 到 为 correct?
- 如何 曾是 the change build tested 和 run-time tested?
- 什么 prior commit 执行 此 change fix? 此 应当 go 在 一个 "Fixes:"
  tag 作为 the documentation describes.
- 谁 else 具有 reviewed 此 patch? 此 应当 go 在 appropriate
  "Reviewed-by:" tags; 参见 下文.

```

  From: Author <author@email>
  Subject: [PATCH] drivers/foo_bar: Add missing kfree()

  The error path in foo_bar driver does not correctly free the allocated
  struct foo_bar_info. This can happen if the attached foo_bar device
  rejects the initialization packets sent during foo_bar_probe(). This
  would result in a 64 byte slab memory leak once per device attach,
  wasting memory resources over time.

  This flaw was found using an experimental static analysis tool we are
  developing, LeakMagic[1], which reported the following warning when
  analyzing the v5.15 kernel release:

   path/to/foo_bar.c:187: missing kfree() call?

  Add the missing kfree() to the error path. No other references to
  this memory exist outside the probe function, so this is the only
  place it can be freed.

  x86_64 and arm64 defconfig builds with CONFIG_FOO_BAR=y using GCC
  11.2 show no new warnings, and LeakMagic no longer warns about this
  code path. As we don't have a FooBar device to test with, no runtime
  testing was able to be performed.

  [1] https://url/to/leakmagic/details

  Reported-by: Researcher <researcher@email>
  Fixes: aaaabbbbccccdddd ("Introduce support for FooBar")
  Signed-off-by: Author <author@email>
  Reviewed-by: Reviewer <reviewer@email>

```
若 您 是 一个 第一 time contributor 它是 recommended 该 the patch
itself 为 vetted 由 others privately 之前 正在 posted 到 公共 列表.
(这是 必需 若 您 具有 已经 explicitly told 您的 patches 需要
更多 careful 内部 review.) 这些 people 是 expected 到 具有 它们的
"Reviewed-by" tag included 在 the resulting patch. Finding another
developer familiar 与 Linux contribution, especially 之内 您的 own
organization, 和 having them help 与 reviews 之前 sending them 到
the 公共 mailing 列表 tends 到 significantly improve the quality 的 the
resulting patches, 和 那里 由 reduces the burden 在 其他 developers.

若 无 one 可 为 found 到 internally review patches 和 您 需要
help finding 此类 一个 person, 或 若 您 具有 任何 其他 questions
related 到 此 document 和 the developer community's expectations,
请 reach out 到 the 私有 Technical Advisory Board mailing 列出:
<tech-board@groups.linuxfoundation.org>.
