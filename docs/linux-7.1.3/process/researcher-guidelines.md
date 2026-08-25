锘。

Researcher Guidelines
+++++++++++++++++++++

The Linux 内核 community welcomes transparent research the Linux
内核, the activities involved producing  任何 其他 byproducts
development. Linux benefits greatly 来自 kind research, 
大多aspects Linux driven research one form another.

The community greatly appreciates 鑻?researchers 鍙?share preliminary
findings 之前 making 它们results 公共, especially 此类 research
involves 安全. Getting involved early helps 两improve the quality
research ability 用于 Linux improve 来自  任何 case,
sharing 打开 access copies the published research the community
鏄?recommended.

document seeks clarify 什the Linux 内核 community considers
acceptable non-acceptable practices conducting 此类 research. 
the very least, 姝ょ被 research 鍜?related activities 搴斿綋 follow
标准 research ethics rules. 用于 更多 background research ethics
generally, ethics 鍦?technology, 鍜?research 鐨?developer communities
特别 参见:

- `History of Research Ethics <https://www.unlv.edu/research/ORI-HSR/history-ethics>`_
- `IEEE Ethics <https://www.ieee.org/about/ethics/index.html>`_
- `Developer and Researcher Views on the Ethics of Experiments on Open-Source Projects <https://arxiv.org/pdf/2112.13217.pdf>`_

The Linux 内核 community expects everyone interacting the
project 鏄?participating 鍦?good faith 鍒?make Linux better. Research 鍦。
任何 publicly-available artifact (including, limited source
code) produced the Linux 内核 community welcome, though research
developers 必须 distinctly opt-in.

Passive research based entirely publicly 可用 sources,
including posts 公共 mailing 列表 commits 公共
repositories, clearly permissible. Though, 作为 任何 research,
标准 ethics 必须 仍然 followed.

Active research developer behavior, 然 必须 已完the
explicit agreement  full disclosure  the 各个 developers
involved. Developers cannot 涓?interacted 涓?experimented 鍦，鏃。
consent;  too, 标准 research ethics.

## Surveys


Research 通常 takes the form surveys sent maintainers 
contributors.  作为 一通用 rule, though, the 内核 community derives
little 来自 这些 surveys.  The 内核 development 进程 works
因为 every developer benefits 来自 它们participation, even working
others 具有 不同 goals.  Responding 一survey, though, 一
one-way demand placed 鍦?busy developers 涓，鏃?corresponding benefit 鍒。
themselves the 内核 community 作为 一whole.  用于 reason, 
方法 research discouraged.

内核 community members 已经 receive far too much email likely
perceive survey requests 作为 just another demand 它们time.  Sending
此类 requests deprives the community valuable contributor time 
unlikely yield 一statistically useful 响应.

作为 一alternative, researchers 应当 consider attending developer 事件,
hosting sessions 何处 the research project benefits the
participants 鍙，涓?explained, 鍜?interacting directly 涓?the community
那里.  The information received far richer obtained 来自
一email survey, the community gain 来自 the ability learn 来自
您的 insights 作为 well.

## Patches


鍒?help clarify: sending patches 鍒?developers **鏄?* interacting
them, 它们 具有 已经 consented receiving *good faith
contributions*. Sending intentionally flawed/vulnerable patches 鎴。
contributing misleading information 鍒?discussions 鏄，涓?consented
 此类 communication damaging the developer (e.g. draining
time, effort, 鍜?morale) 鍜?damaging 鍒?the project 鐢?eroding
the entire developer community's trust 鍦?the contributor (鍜?the
contributor's organization 作为 一whole), undermining efforts 提供
constructive feedback 鍒?contributors, 鍜?putting end users 鍦?risk 鐨。
软件 flaws.

Participation the development Linux itself researchers, 作为
anyone, welcomed encouraged. Research 进入 Linux code 
一通用 practice, especially comes developing 运行
analysis tools 璇?produce actionable results.

engaging the developer community, sending 一patch 具有
traditionally 已经 the best way make 一impact. Linux 已经 具有
plenty known bugs -- 什s much 更多 helpful having vetted fixes.
之前 contributing, carefully 读取 the appropriate documentation:

- Documentation/进程/development-process.rst
- Documentation/进程/submitting-patches.rst
- Documentation/admin-guide/reporting-issues.rst
- Documentation/进程/security-bugs.rst

然后 send 一patch (including 一commit log 全部 the details listed
下文) follow up 任何 feedback 来自 其他 developers.

sending patches produced 来自 research, the commit logs 应当
包含 至少 the 以下 details, 因此 developers 具有
appropriate 上下用于 understanding the contribution. Answer:

- 什the 特定 problem 具有 已经 found
- 如何 可以 the problem reached 一运行系系
- 什effect 将会 encountering the problem 具有 the 系系
- 如何 曾是 the problem found? Specifically 包含 details 关于 任何
  testing, 静动analysis programs, 任何 其他 tools 
  方法 使用 perform the work.
- 版本 Linux 曾是 the problem found  使用 the 大多recent
  释放 一recent linux-next branch strongly preferred (参见
  Documentation/进程/howto.rst).
- 什曾是 changed fix the problem, 为何 它是 believed correct
- 濡備綍 鏇炬槸 the change build tested 鍜?run-time tested?
- 什prior commit 执行 change fix 应当 go 一"Fixes:"
  tag 作为 the documentation describes.
- else 具有 reviewed patch 应当 go appropriate
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
一第一 time contributor 它是 recommended the patch
itself vetted others privately 之前 正在 posted 公共 列表.
(这是 必需 具有 已经 explicitly told 您的 patches 需
更多 careful 内部 review.) 这些 people expected 具有 它们
"Reviewed-by" tag included 鍦?the resulting patch. Finding another
developer familiar Linux contribution, especially 之内 您的 own
organization, having them help reviews 之前 sending them 
the 公共 mailing 列表 tends significantly improve the quality the
resulting patches, 那里 reduces the burden 其他 developers.

one found internally review patches 需
help finding 此类 一person, 具有 任何 其他 questions
related 鍒，姝?document 鍜?the developer community's expectations,
reach out the 私有 Technical Advisory Board mailing 列出:
<tech-board@groups.linuxfoundation.org>.
