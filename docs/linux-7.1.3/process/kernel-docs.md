
## 更多内核文档索引


linux-kernel 邮件列表中，由于反复出现同样的、寻求资料指引的问题，编写这样一份文档的必要性变得显而易见
幸运的是，随着使用 GNU/Linux 的人越来越多，对内核感兴趣的人也越来越多。但仅仅阅读源代码并不总是足够。理解代码本身很容易，却容易忽略其背后的概念、理念与设计决策
遗憾的是，可供初学者入门的文档并不多。而且即便存在，此前也没有一个“广为人知”的地方来追踪它们。这些内容正是为了弥补这一缺憾
如果你知道任何此处未列出的文章，或者撰写了新的文档，请遵循内核的补丁提交流程，在此处添加对它的引用。同时也欢迎任何更正、想法或评论
所有文档均按以下字段编目：文档的“标题（Title）”、作者（Author）、“URL”所在位置、有助于检索特定主题的若干“关键词（Keywords）”，以及对文档的简短“描述（Description）”

   本文档每个小节中的文档均按发布日期由新到旧排列。维护者应当定期将已过时或陈旧的资源下架，基础著作除外
### 位于 Linux 内核源码树中的文

Sphinx 书籍应使`make {htmldocs | pdfdocs | epubdocs}` 构建
    - Name: **linux/Documentation**

      :Author: Many.
      :Location: Documentation/
      :Keywords: text files, Sphinx.
      :Description: 随内核源码一同提供的文档，位Documentation 目录内。本文档中的部分页面（包括本文档本身）已迁移至此，可能比 Web 版本更为更新
### 在线文档


    - Title: **Linux Kernel Mailing List Glossary**

      :Author: various
      :URL: https://kernelnewbies.org/KernelGlossary
      :Date: rolling version
      :Keywords: glossary, terms, linux-kernel.
      :Description: 引自简介：“本术语表旨在简要说明在讨论 Linux 内核时你可能听到的一些缩写与术语”
    - Title: **The Linux Kernel Module Programming Guide**

      :Author: Peter Jay Salzman, Michael Burian, Ori Pomerantz, Bob Mottram,
        Jim Huang.
      :URL: https://sysprog21.github.io/lkmpg/
      :Date: 2021
      :Keywords: modules, GPL book, /proc, ioctls, system calls,
        interrupt handlers .
      :Description: 一本非常不错的关于模块编程主题GPL 书籍。包含大量示例。目前新版本正活跃地维护https://github.com/sysprog21/lkmpg
### 已出版书

    - Title: **The Linux Memory Manager**

      :Author: Lorenzo Stoakes
      :Publisher: No Starch Press
      :Date: February 2025
      :Pages: 1300
      :ISBN: 978-1718504462
      :Notes: 内存管理。完整草稿以早期访问形式提供用于预售，完整版计划2025 年秋季发布。详https://nostarch.com/linux-memory-manager
    - Title: **Practical Linux System Administration: A Guide to Installation, Configuration, and Management, 1st Edition**

      :Author: Kenneth Hess
      :Publisher: O'Reilly Media
      :Date: May, 2023
      :Pages: 246
      :ISBN: 978-1098109035
      :Notes: 系统管理

    - Title: **Linux Kernel Debugging: Leverage proven tools and advanced techniques to effectively debug Linux kernels and kernel modules**

      :Author: Kaiwan N Billimoria
      :Publisher: Packt Publishing Ltd
      :Date: August, 2022
      :Pages: 638
      :ISBN: 978-1801075039
      :Notes: 调试相关书籍

    - Title: **Linux Kernel Programming: A Comprehensive Guide to Kernel Internals, Writing Kernel Modules, and Kernel Synchronization**

      :Author: Kaiwan N Billimoria
      :Publisher: Packt Publishing Ltd
      :Date: March, 2021 (Second Edition published in 2024)
      :Pages: 754
      :ISBN: 978-1789953435 (Second Edition ISBN is 978-1803232225)

    - Title: **Linux Kernel Programming Part 2 - Char Device Drivers and Kernel Synchronization: Create user-kernel interfaces, work with peripheral I/O, and handle hardware interrupts**

      :Author: Kaiwan N Billimoria
      :Publisher: Packt Publishing Ltd
      :Date: March, 2021
      :Pages: 452
      :ISBN: 978-1801079518

    - Title: **Linux System Programming: Talking Directly to the Kernel and C Library**

      :Author: Robert Love
      :Publisher: O'Reilly Media
      :Date: June, 2013
      :Pages: 456
      :ISBN: 978-1449339531
      :Notes: 基础著作

    - Title: **Linux Kernel Development, 3rd Edition**

      :Author: Robert Love
      :Publisher: Addison-Wesley
      :Date: July, 2010
      :Pages: 440
      :ISBN: 978-0672329463
      :Notes: 基础著作


    - Title: **Linux Device Drivers, 3rd Edition**

      :Authors: Jonathan Corbet, Alessandro Rubini, and Greg Kroah-Hartman
      :Publisher: O'Reilly & Associates
      :Date: 2005
      :Pages: 636
      :ISBN: 0-596-00590-3
      :Notes: 基础著作。更多信息见 http://www.oreilly.com/catalog/linuxdrive3/（PDF 格式），URL：https://lwn.net/Kernel/LDD3/

    - Title: **The Design of the UNIX Operating System**

      :Author: Maurice J. Bach
      :Publisher: Prentice Hall
      :Date: 1986
      :Pages: 471
      :ISBN: 0-13-201757-1
      :Notes: 基础著作

### 其他


    - Name: **Cross-Referencing Linux**

      :URL: https://elixir.bootlin.com/
      :Keywords: Browsing source code.
      :Description: 另一个基Web Linux 内核源代码浏览器。包含大量指向变量和函数的交叉引用。你可以看到它们在何处定义、在何处使用
    - Name: **Linux Weekly News**

      :URL: https://lwn.net
      :Keywords: latest kernel news.
      :Description: 标题即说明一切。其中有一个固定的内核栏目，汇总开发者在一周内完成的工作、缺陷修复、新特性与新版本
    - Name: **The home page of Linux-MM**

      :Author: The Linux-MM team.
      :URL: https://linux-mm.org/
      :Keywords: memory management, Linux-MM, mm patches, TODO, docs,
        mailing list.
      :Description: 致力Linux 内存管理开发的站点。内存相关的补丁、HOWTO、链接、mm 开发者……如果你对内存管理开发感兴趣，千万别错过
    - Name: **Kernel Newbies IRC Channel and Website**

      :URL: https://www.kernelnewbies.org
      :Keywords: IRC, newbies, channel, asking doubts.
      :Description: irc.oftc.net 上的 #kernelnewbieskernelnewbies 是一个致力于“新手”内核黑客的 IRC 网络。其成员大多是在学习内核、从事内核项目的人，或是希望帮助经验较少的内核开发者的专业内核黑客kernelnewbies 位于 OFTC IRC 网络。请irc.oftc.net 作为服务器，然后执行 /join #kernelnewbies。kernelnewbies 网站还托管了文章、文档、FAQ…
    - Name: **linux-kernel mailing list archives and search engines**

      :URL: https://subspace.kernel.org
      :URL: https://lore.kernel.org
      :Keywords: linux-kernel, archives, search.
      :Description: linux-kernel 邮件列表的部分归档器。如果你有更其他的归档器，请告知我
    - Name: **The Linux Foundation YouTube channel**

      :URL: https://www.youtube.com/user/thelinuxfoundation
      :Keywords: linux, videos, linux-foundation, youtube.
      :Description: Linux 基金会上传其协作活动、包LinuxCon 在内Linux 会议的视频录像，以及其他Linux 和软件开发相关的原创研究与内容
### Rust


    - Title: **Rust for Linux**

      :Author: various
      :URL: https://rust-for-linux.com/
      :Date: rolling version
      :Keywords: glossary, terms, linux-kernel, rust.
      :Description: 引自网站：“Rust for Linux 是一个为 Linux 内核添加 Rust 语言支持的项目。本网站旨在作为与该项目的链接、文档和资源相关的中枢”
    - Title: **Learn Rust the Dangerous Way**

      :Author: Cliff L. Biffle
      :URL: https://cliffle.com/p/dangerust/
      :Date: Accessed Sep 11 2024
      :Keywords: rust, blog.
      :Description: 引自网站：“LRtDW 是一系列文章，为可能没有正式计算机科学背景的底层 C 程序员介Rust 特性——也就是那些从事固件、游戏引擎、操作系统内核等工作的人。基本上，就是像我这样的人。它以逐行的方式展示了C Rust 的转换
    - Title: **The Rust Book**

      :Author: Steve Klabnik and Carol Nichols, with contributions from the
        Rust community
      :URL: https://doc.rust-lang.org/book/
      :Date: Accessed Sep 11 2024
      :Keywords: rust, book.
      :Description: 引自网站：“本书充分发Rust 的潜力，以赋能其用户。这是一本友好且易于理解的文本，旨在帮助你不仅提Rust 知识，也提升作为程序员整体的能力范围与信心。那就开始吧，准备好学习——欢迎来Rust 社区！”
    - Title: **Rust for the Polyglot Programmer**

      :Author: Ian Jackson
      :URL: https://www.chiark.greenend.org.uk/~ianmdlvl/rust-polyglot/index.html
      :Date: December 2022
      :Keywords: rust, blog, tooling.
      :Description: 引自网站：“关Rust 的指南与入门有很多。这一份有所不同：它面向已经掌握多种其他编程语言的资深程序员。我力求内容足够全面，可作为任意 Rust 领域的起点，但除了那些与预期不符之处外，避免过于深入细节。此外，本指南也并非完全没有个人观点，包括对库（crates）、工具链等的推荐。
    - Title: **Fasterthanli.me**

      :Author: Amos Wenger
      :URL: https://fasterthanli.me/
      :Date: Accessed Sep 11 2024
      :Keywords: rust, blog, news.
      :Description: 引自网站：“我制作关于计算机如何运作的文章和视频。我的内容篇幅较长、具有教学性和探索性——而且常常成了讲解 Rust 的契机！”
    - Title: **Comprehensive Rust**

      :Author: Android team at Google
      :URL: https://google.github.io/comprehensive-rust/
      :Date: Accessed Sep 13 2024
      :Keywords: rust, blog.
      :Description: 引自网站：“本课程涵盖 Rust 的方方面面，从基础语法到泛型与错误处理等高级主题”
    - Title: **The Embedded Rust Book**

      :Author: Multiple contributors, mostly Jorge Aparicio
      :URL: https://docs.rust-embedded.org/book/
      :Date: Accessed Sep 13 2024
      :Keywords: rust, blog.
      :Description: 引自网站：“一本关于在微控制器等‘裸机（Bare Metal）’嵌入式系统上使Rust 编程语言的入门书籍”
   - Title: **Experiment: Improving the Rust Book**

      :Author: Cognitive Engineering Lab at Brown University
      :URL: https://rust-book.cs.brown.edu/
      :Date: Accessed Sep 22 2024
      :Keywords: rust, blog.
      :Description: 引自网站：“本实验的目标是评估并改进《Rust Book》的内容，以帮助人们更有效地学习 Rust。”
   - Title: **New Rustacean** (podcast)

      :Author: Chris Krycho
      :URL: https://newrustacean.com/
      :Date: Accessed Sep 22 2024
      :Keywords: rust, podcast.
      :Description: 引自网站：“这是一档关于从零开始学Rust 编程语言的播客！除了这个漂亮的着陆页之外，网站所有内容都是用 Rust 自带的文档工具构建的。”
   - Title: **Opsem-team** (repository)

      :Author: Operational semantics team
      :URL: https://github.com/rust-lang/opsem-team/tree/main
      :Date: Accessed Sep 22 2024
      :Keywords: rust, repository.
      :Description: 引自 README：“opsem 团队unsafe-code-guidelines 工作组的继任者，负责回答关于 unsafe Rust 语义的诸多难题”
    - Title: **You Can't Spell Trust Without Rust**

      :Author: Alexis Beingessner
      :URL: https://repository.library.carleton.ca/downloads/1j92g820w?locale=en
      :Date: 2015
      :Keywords: rust, master, thesis.
      :Description: 本论文聚焦于 Rust 的所有权系统，该系统通过控制数据操作与生命周期来保证内存安全，同时也指出了其局限性，并将其与 Cyclone C++ 中的类似系统进行比较
    - Name: **Linux Plumbers (LPC) 2024 Rust presentations**

      :Title: Rust microconference
      :URL: https://lpc.events/event/18/sessions/186/#20240918
      :Title: Rust for Linux
      :URL: https://lpc.events/event/18/contributions/1912/
      :Title: Journey of a C kernel engineer starting a Rust driver project
      :URL: https://lpc.events/event/18/contributions/1911/
      :Title: Crafting a Linux kernel scheduler that runs in user-space
        using Rust
      :URL: https://lpc.events/event/18/contributions/1723/
      :Title: openHCL: A Linux and Rust based paravisor
      :URL: https://lpc.events/event/18/contributions/1956/
      :Keywords: rust, lpc, presentations.
      :Description: 若干Rust 相关LPC 演讲
    - Name: **The Rustacean Station Podcast**

      :URL: https://rustacean-station.org/
      :Keywords: rust, podcasts.
      :Description: 一个为 Rust 编程语言创作播客内容的社区项目
-------

本文档最初基于：

 https://www.dit.upm.es/~jmseyas/linux/kernel/hackers-docs.html

Juan-Mariano de Goyeneche 撰写

