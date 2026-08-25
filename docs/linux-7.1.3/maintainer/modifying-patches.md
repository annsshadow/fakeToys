
## 修改补丁（Modifying Patches

如果你是某个子系统或分支的维护者，有时你需要对自己收到的补丁进行轻微修改才能合并它们，因为你的代码树与提交者的代码树并不完全一样。如果你严格遵循开发者来源证书（developers certificate of origin）的规则 (c)，你应该要求提交者重新生diff，但这完全是一种事倍功半的浪费时间和精力。规(b) 允许你调整代码，但修改他人代码并让其为你引入bug 背书是非常不礼貌的。为解决此问题，建议你在最后一Signed-off-by 头部与你自己的头部之间加一行，说明你修改的性质。虽然这并非强制要求，但似乎将描述用你的邮件或姓名括在方括号中作为前缀，已足够明显地使其一目了```

       Signed-off-by: Random J Developer <random@developer.example.org>
       [lucky@maintainer.example.org: struct foo moved from foo.c to foo.h]
       Signed-off-by: Lucky K Maintainer <lucky@maintainer.example.org>

```
这种做法在以下情况特别有帮助：你维护一个稳定分支，同时想为作者记功、跟踪变更、合并修复，并保护提交者免受抱怨。注意，在任何情况下你都不得更改作者的身份（From 头部），因为那才是出现在变更日志中的身份
给向后移植者（back-porter）的特别提示：在提交消息顶部（主题行之后）插入补丁来源指示以方便跟踪，似乎是一种常见且有用的做法。例如，
```

  Date:   Tue Oct 7 07:26:38 2014 -0400

    libata: Un-break ATA blacklist

    commit 1c40279960bcd7d52dbdf1d466b20d24b99176c8 upstream.

```
```

    Date:   Tue May 13 22:12:27 2008 +0200

        wireless, airo: waitbusy() won't delay

        [backport of 2.6 commit b7acbdfbd1f277c1eb23f344f899cfa4cd0bf36a]

```
无论何种格式，这些信息都为跟踪你代码树的人以及排查你代码树中 bug 的人提供了宝贵的帮助