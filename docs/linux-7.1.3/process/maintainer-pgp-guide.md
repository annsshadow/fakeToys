
## 鍐呮牳缁存姢鑰?PGP 鎸囧崡


:Author: Konstantin Ryabitsev <konstantin@linuxfoundation.org>

本文档面Linux 内核开发者，尤其是子系统维护者。其中包含了Linux
基金会发布的更通用“`Protecting Code Integrity`_"（保护代码完整性）
指南中所讨论信息的一个子集。若想就本指南中提到的某些主题获得更深入讨论，请阅读该文档

## PGP Linux 内核开发中的作

PGP 有助于确保由 Linux 内核开发社区所产生的代码的完整性，并在较小程度通过 PGP 签名的电子邮件交换在开发者之间建立可信的通信渠道
Linux 内核源代码主要以两种形式提供
- 分布式源代码仓库（git- 周期性发布的快照（tarball
git 仓库tarball 都带有创建官方内核发布的那些内核开发者的 PGP 签名这些签名提供了一种密码学保证，即经由 kernel.org 或任何其他镜像提供的
可下载版本，与这些开发者在其工作站上拥有的版本完全一致。为此：

- git 仓库在所有标签（tag）上提供 PGP 签名
- tarball 在所有下载内容中提供独立的（detached）PGP 签名


### 信任开发者，而非基础设施


2011 年核kernel.org 系统被攻陷以来，Kernel Archives 项目的主运作原则一直是：假定基础设施的任何部分都可能在任何时候被攻陷。出于这原因，管理员们采取了审慎的措施来强调：信任必须始终放在开发者身上，绝不放在代码托管基础设施上，无论后者的安全实践做得有多好
上述指导原则正是本指南存在的原因。我们希望确保在将信任寄托于开发者时不会仅仅是将未来潜在安全事件的责任推卸给他人。我们的目标是提供一准则，开发者可以用它们来创建一个安全的工作环境，并保护用于确立 Linux
内核自身完整性的 PGP 密钥

## PGP 工具


### 使用 GnuPG 2.4 或更高版

您的发行版应该已经默认安装了 GnuPG，您只需验证自己使用的是一个相较新的版本```

    $ gpg --version | head -n1

```
如果您使用的2.4 或更高版本，那么您就可以直接开始。如果您使用的是
更早的版本，那么您正在使用一个已不再受维护的 GnuPG 版本，本指南中的
某些命令可能无法工作
#### 配置 gpg-agent 选项


GnuPG agent 是一个辅助工具，会在您每次使`gpg` 命令时自动启动，并在
后台运行，目的是缓存私钥的口令（passphrase）。为了调整口令从缓存过期的时间，您需要了解以下两个选项
- `default-cache-ttl`（秒）：如果您在该生存时间（time-to-live）到  之前再次使用同一个密钥，倒计时将为下一个周期重置。默认值为 600
  0 分钟）- `max-cache-ttl`（秒）：无论您自初次输入口令以来最近是否使用过该密钥，
  如果最大生存时间倒计时到期，您都将被要求再次输入口令。默认值为 30
  分钟
如果您觉得其中任一默认值太短（或太长），您可以
```

    # 将常ttl 设为 30 分钟，最ttl 设为 2 小时
    default-cache-ttl 1800
    max-cache-ttl 7200

```

    在您shell 会话开始时，不再需要手动启gpg-agent。您可能想要
    检查您rc 文件，移除为旧版GnuPG 而设置的任何内容，因为它可能
    不再做正确的事情了

## 保护您的 PGP 密钥


本指南假定您已经拥有用于 Linux 内核开发目的的 PGP 密钥。如果您还没有，
请参阅前面提到的 “`Protecting Code Integrity`_" 文档，以获取关于
如何创建新密钥的指导
如果您当前的密钥弱于 2048 位（RSA），您也应该创建一个新密钥
### 鐞嗚В PGP 瀛愬瘑閽?

一PGP 密钥很少仅由单个密钥对组成——通常它是一组相互独立的子密（subkey）的集合，这些子密钥可以根据其在创建时分配的能力用于不同目的。PGP 定义了一个密钥可以拥有的四种能力
- **[S]** 密钥可用于签- **[E]** 密钥可用于加- **[A]** 密钥可用于认- **[C]** 密钥可用于认证（certify）其他密
带有 **[C]** 能力的密钥通常被称为“主（master）”密钥，但这个术语具误导性，因为它暗Certify 密钥可以替代同一密钥链上的任何其他子密钥
（就像一个物理上的“主钥匙”可以用来打开为其他钥匙制作的锁）。由于事并非如此，本指南将称其为“Certify 密钥”，以避免任何歧义
充分理解以下几点至关重要
1. 所有子密钥彼此完全独立。如果您丢失了一个私钥子密钥，它无法从您
   密钥链上的任何其他私钥中恢复或重建2. Certify 密钥外，可以存在多个具有相同能力的子密钥（例如，   可以拥有 2 个有效的加密子密钥 个有效的签名子密钥，但只1    有效的认证子密钥）。所有子密钥都完全独立——加密给某个 **[E]** 子密   的邮件，无法用您可能拥有的任何其**[E]** 子密钥解密3. 单个子密钥可以拥有多种能力（例如，您**[C]** 密钥也可以同时是
   您的 **[S]** 密钥
带有 **[C]**（certify，认证）能力的密钥是唯一可用于表明与其他密钥
关系的密钥。只**[C]** 密钥可以用于
- 添加或撤销带有 S/E/A 能力的其他密钥（子密钥）
- 添加、更改或撤销与该密钥关联的标识（uid- 添加或更改其自身或任何子密钥的过期日- 出于信任网（web of trust）目的而签署其他人的密
默认情况下，GnuPG 在生成新密钥时会创建以下内容
- 一个同时带Certify Sign 能力的子密钥*[SC]**- 一个带Encryption 能力的独立子密钥*[E]**
如果您在生成密钥时使用了默认参数，那么您将拥有上述内容。您可以通过运行
`gpg --list-secret-keys` 来验证，
```

    sec   ed25519 2022-12-20 [SC] [expires: 2024-12-19]
          000000000000000000000000AAAABBBBCCCCDDDD
    uid           [ultimate] Alice Dev <adev@kernel.org>
    ssb   cv25519 2022-12-20 [E] [expires: 2024-12-19]

```
`sec` 条目下方的长行就是您的密钥指纹（fingerprint）—在下面的示例中，每当您看`[fpr]` 时，指的就是那个 40 字符的字符串
### 确保您的口令足够强壮


GnuPG 在将私钥存储到磁盘之前，使用口令对其进行加密。这样，即使您的
`.gnupg` 目录被整体泄露或窃取，攻击者在首先获取到用于解密的口令之前也无法使用您的私钥
让您的私钥受到保护绝对至关重要，方法是使用足够强壮的
```

    $ gpg --change-passphrase [fpr]

```

### 创建一个独立的签名子密

我们的目标是将您Certify 密钥移动到离线介质上加以保护，因此如果您
只有一个组合的 **[SC]** 密钥，那么您应该创建一```

    $ gpg --quick-addkey [fpr] ed25519 sign

```

### 备份您的 Certify 密钥以应对灾难恢

您从其他开发者那里获得的、对PGP 密钥的签名越多，您就越有理由创建一存放在数字介质之外的某种介质上的备份版本，以应对灾难恢复的需要
创建私钥可打印硬拷贝的一个好方法是使用为此目的而编写的 `paperkey`
软件。有关输出格式及其相对于其他解决方案的优势，请参``man
paperkey``。paperkey 应该已经为大多数发行版打包好了
运行以下命令来创建您私钥的硬拷贝备份```

    $ gpg --export-secret-key [fpr] | paperkey -o /tmp/key-backup.txt

```
打印出该文件，然后拿起一支笔，将您的口令写在纸张的页边。这**强烈推荐**
这样做，因为密钥打印件仍然是用该口令加密的，并且如果您日后更改了口令您将无法记得创建备份时它曾是什么—*这是必然*
将得到的打印件和手写的口令放入一个信封中，并存放在一个安全且受到良好
保护的地方，最好是远离您的住所，例如您的银行保险箱

    密钥仍然是用您的口令加密的，因此即使打印到“云集成”的现代打印机，
    也应当是一个相对安全的操作
### 备份您的整个 GnuPG 目录



    **!!!请勿跳过此步!!**

拥有一份随时可用的 PGP 密钥备份非常重要，以备您需要恢复它们之时。这我们之前`paperkey` 所做的灾难级准备不同。您还将在每次需要使Certify
密钥时依赖这些外部副本——例如在修改您自己的密钥之后，或在会议与峰会签署其他人的密钥时
首先获取一个用于备份目的的外置介质卡（最好是两个！）。您将需要在此设上使LUKS 创建一个加密分区——请参阅您的发行版文档以了解如何完成
对于加密口令，您可以使用PGP 密钥上相同的那个
加密过程结束后，重新插入您的设备并确保它被正确地挂载。将您整`.gnupg` 目录复制```

    $ cp -a ~/.gnupg /media/disk/foo/gnupg-backup

```

```

    $ gpg --homedir=/media/disk/foo/gnupg-backup --list-key [fpr]

```
如果您没有收到任何错误，那么您就可以直接开始。卸载该设备，清晰地为其
贴上标签，以免意外覆盖它，并存放在安全的地方——但不要放得太远，因为您
每隔一段时间就需要用到它，比如编辑标识、添加或撤销子密钥，或者签其他人的密钥
### 从您的主目录中移Certify 密钥


我们主目录中的文件并不像我们愿意认为的那样受到良好保护。它们可能通过
多种不同的方式被泄露或窃取：

- 在为设置新工作站而快速复制主目录时意外发- 由系统管理员疏忽或恶意造成
- 经由保护不佳的备- 经由桌面应用中的恶意软件（浏览器、pdf 查看器等- 经由跨越国际边境时的强迫

用一个好的口令保护您的密钥，极大地有助于降低上述任何情况的风险，口令可能通过键盘记录器、肩窥（shoulder-surfing）或任何其他许多手段
被发现。出于这个原因，推荐的配置是从您的主目录中移Certify 密钥并将其存储在离线存储中

    请参阅上一节，并确保您已经整体备份了您GnuPG 目录。如果您没有
    可用的备份，我们即将要做的事情会让您的密钥变得毫无用处！

```

    $ gpg --with-keygrip --list-key [fpr]

```

```

    pub   ed25519 2022-12-20 [SC] [expires: 2022-12-19]
          000000000000000000000000AAAABBBBCCCCDDDD
          Keygrip = 1111000000000000000000000000000000000000
    uid           [ultimate] Alice Dev <adev@kernel.org>
    sub   cv25519 2022-12-20 [E] [expires: 2022-12-19]
          Keygrip = 2222000000000000000000000000000000000000
    sub   ed25519 2022-12-20 [S]
          Keygrip = 3333000000000000000000000000000000000000

```
找到位于 `pub` 行下方（紧挨 Certify 密钥指纹下方）的 keygrip 条目这将直接对应于您
```

    $ cd ~/.gnupg/private-keys-v1.d
    $ ls
    1111000000000000000000000000000000000000.key
    2222000000000000000000000000000000000000.key
    3333000000000000000000000000000000000000.key

```
中的某个文件。移除与 Certify 密钥对应的那.key 文件即可```

    $ cd ~/.gnupg/private-keys-v1.d
    $ rm 1111000000000000000000000000000000000000.key

```
现在，如果您发出 `--list-secret-keys` 命令，它将显```

    $ gpg --list-secret-keys
    sec#  ed25519 2022-12-20 [SC] [expires: 2024-12-19]
          000000000000000000000000AAAABBBBCCCCDDDD
    uid           [ultimate] Alice Dev <adev@kernel.org>
    ssb   cv25519 2022-12-20 [E] [expires: 2024-12-19]
    ssb   ed25519 2022-12-20 [S]

```
您还应该移除 `~/.gnupg` 目录中任何遗留的 `secring.gpg` 文件，它们可是以前版本的 GnuPG 留下的
#### 如果您没“private-keys-v1.d目录


如果您没`~/.gnupg/private-keys-v1.d` 目录，那么您的私钥仍存储GnuPG v1 使用的旧`secring.gpg` 文件中。对您的密钥做任何更改，例如
更改口令或添加子密钥，都应该会自动将旧的 `secring.gpg` 格式转换为使`private-keys-v1.d`
一旦您完成上述操作，请务必删除已废弃的 `secring.gpg` 文件，它仍然包含
您的私钥

## 将子密钥移动到专用加密设

尽管 Certify 密钥现在已不会被泄露或窃取，但子密钥仍在您的主目录中任何设法拿到它们的人都将能够解密您的通信，或在知道口令的情况下伪造您签名。此外，每次执行 GnuPG 操作时，密钥都会被加载到系统内存中，并可被足够高级的恶意软件（想Meltdown Spectre）从那里窃取
完全保护您的密钥的一个好方法是将它们移动到一个能够执行智能卡（smartcard操作的专用硬件设备上
### 智能卡的好处


智能卡包含一个加密芯片，能够存储私钥并直接在卡上执行加密操作。由于密内容永远不会离开智能卡，插入该硬件设备的计算机的操作系统无法获取私钥
本身。这与我们之前用于备份目的的加密介质存储设备非常不同——当该设插入并挂载时，操作系统能够访问私钥内容
使用外置加密介质并不能替代拥有支持智能卡的设备
### 可用的智能卡设备


除非您所有的笔记本和工作站都有智能卡读卡器，否则最简单的办法是获取一实现智能卡功能的专用 USB 设备。有几种选择可用
- `Nitrokey Start`_：开放的硬件和自由软件（Free Software），基于 FSI
  Japan `Gnuk`_。最便宜的选择之一，但提供最少的安全特性（例如抗篡  或某些侧信道攻击的能力）- `Nitrokey 3`_：类Nitrokey Start，但更抗篡改，并提供更多安全特性和
  USB 外形规格。支ECC 加密（ED25519 NISTP）- `Yubikey 5`_：专有硬件和软件，但Nitrokey 便宜，具有类似的一组特性  支持 ECC 加密（ED25519 NISTP）
您的选择将取决于成本、您所在地理区域的供货可用性，以及开专有硬件
方面的考量

    如果您在 MAINTAINERS 中被列为 `M:` 条目，或者在 kernel.org 拥有
    账户，那么您就有资格免费获得Linux 基金会提供的 `qualify for a free Nitrokey Start`_
### 配置您的智能卡设

您的智能卡设备应该在插入的那一刻就能“直接工作”（Just Work (TM)）：
```

    $ gpg --card-status

```
如果您看到了完整的智能卡详情，那么您就可以直接开始
遗憾的是，排查所有可能导致设备无法为您工作的可能原因，远远超出了本指的范围。如果您在让卡片GnuPG 配合工作方面遇到麻烦，请通过常规的支渠道寻求帮助
要配置您的智能卡，您需要使GnuPG 菜单系统，例```

    $ gpg --card-edit
    [...omitted...]
    gpg/card> admin
    Admin commands are allowed
    gpg/card> passwd

```
您应该设置用PIN）、Admin PIN）和重置码（Reset Code）。请
务必将这些记录在安全的地方——尤其是 Admin PIN 和重置码（重置码允许完全擦除智能卡）。您极少需要使Admin PIN，因此如果您不记录它，将
不可避免地忘记它是什么
回到主卡片菜单，您还可以设置其他值（例如姓名、性别、登录数据等），但这
并非必要，并且如果您丢失卡片，还会额外泄露关于您智能卡的信息

    尽管名为“PIN”，但卡上的用户 PIN 和管理员 PIN 都不必是数字

    某些设备可能要求您先将子密钥移动到设备上，然后才能更改口令。请查阅
    设备制造商提供的文档
### 将子密钥移动到您的智能卡


退出卡片菜单（使用“q”）并保存所有更改。接下来，让我们将您的子密钥移动智能卡上。您将同时需要您PGP 密钥
```

    $ gpg --edit-key [fpr]

    Secret subkeys are available.

    pub  ed25519/AAAABBBBCCCCDDDD
         created: 2022-12-20  expires: 2024-12-19  usage: SC
         trust: ultimate      validity: ultimate
    ssb  cv25519/1111222233334444
         created: 2022-12-20  expires: never       usage: E
    ssb  ed25519/5555666677778888
         created: 2017-12-07  expires: never       usage: S
    [ultimate] (1). Alice Dev <adev@kernel.org>

    gpg>

```
使用 `--edit-key` 让我们再次进入菜单模式，您会注意到密钥列表略有不同从这里开始，所有命令都在该菜单模式内完成，`gpg>` 所示
首先，让我们选择将要放到卡上的密钥——您通过键入 `key 1` 来完成（它是
列表中的第一个，**[E]**
```

    gpg> key 1

```
在输出中，您现在应该**[E]** 密钥上看`ssb*`。`*` 表示当前“被选中的密钥。它的作用类似于一*开关（toggle*，意味着如果您再次键`key 1`，`*` 将消失，该密钥将不再被选中
```

    gpg> keytocard
    Please select where to store the key:
       (2) Encryption key
    Your selection? 2

```
由于它是我们**[E]** 密钥，将其放入加密槽是合理的。当您提交选择时，
将首先提示您输入 PGP 密钥口令，然后提示输入管理员 PIN。如果命令无错误返回，您的密钥就已经被移动了
**重要**：现在再次键`key 1` 以取消选择第一个密钥，然后
```

    gpg> key 1
    gpg> key 2
    gpg> keytocard
    Please select where to store the key:
       (1) Signature key
       (3) Authentication key
    Your selection? 1

```
您可以将 **[S]** 密钥同时用于签名和认证，但我们要确保它位于签名槽中，
因此选择 (1)。再次，如果您的命令无错误地返回，那么该操作```

    gpg> q
    Save changes? (y/N) y

```
保存更改将删除您移动到卡上的密钥（从您的主目录中）——但这没关系，因如果需要为更换智能卡再次这样做，我们在备份中有它们
#### 验证密钥已被移动


如果您现在执`--list-secret-keys`，您会看到一个细```

    $ gpg --list-secret-keys
    sec#  ed25519 2022-12-20 [SC] [expires: 2024-12-19]
          000000000000000000000000AAAABBBBCCCCDDDD
    uid           [ultimate] Alice Dev <adev@kernel.org>
    ssb>  cv25519 2022-12-20 [E] [expires: 2024-12-19]
    ssb>  ed25519 2022-12-20 [S]

```
`ssb>` 输出中的 `>` 表示该子密钥仅在智能卡上可用。如果您回到您的私钥
目录并查看其中的内容，您会注意到
```

    $ cd ~/.gnupg/private-keys-v1.d
    $ strings *.key | grep 'private-key'

```
输出应包`shadowed-private-key`，以表明这些文件只是桩（stub），实际
内容位于智能卡上
#### 验证智能卡是否正常工

要验证智能卡是否按预期工作，您可以创建一```

    $ echo "Hello world" | gpg --clearsign > /tmp/test.asc
    $ gpg --verify /tmp/test.asc

```
这应该在您的第一个命令中要求输入智能PIN，然后在您运`gpg --verify`
之后显示“Good signature”（良好签名）
恭喜，您已成功让自己的数字开发者身份极难被盗用
### 其他常见GnuPG 操作


以下是您需要使PGP 密钥执行的一些常见操作的快速参考
#### 挂载您的离线安全存储


您将需Certify 密钥来执行下面的任何操作，因此您首先需要挂载您的离备份存储，并告诉 GnuPG 使用
```

    $ export GNUPGHOME=/media/disk/foo/gnupg-backup
    $ gpg --list-secret-keys

```
您需要确保输出中看到的是 `sec` 而非 `sec#`（其中的 `#` 表示密钥不可用，
您仍在使用常规的主目录位置）
#### 延长密钥过期日期


Certify 密钥的默认过期日期为创建之日2 年。这样做既是出于安全考虑，也
是为了让过时的密钥最终从密钥服务器上消失
要将您密钥的过期时间从当前日期起延长一年，只需
```

    $ gpg --quick-set-expire [fpr] 1y

```
如果您觉得使用具体日期更容易记住（例```

    $ gpg --quick-set-expire [fpr] 2038-07-01

```

```

    $ gpg --send-key [fpr]

```

#### 任何更改后更新您的工作目

在使用离线存储对您的密钥做任何更改之后，您将
```

    $ gpg --export | gpg --homedir ~/.gnupg --import
    $ unset GNUPGHOME

```

#### 通过 ssh 使用 gpg-agent


如果您需要在远程系统上签署标签或提交，您可以通过 ssh 转发您的 gpg-agent请参GnuPG wiki 上提供的说明
- `Agent Forwarding over SSH`_

如果您可以修改远程端sshd 服务器设置，它会工作得更顺畅

## Git 中使PGP


Git 的核心特性之一是其去中心化的本质——一旦仓库被克隆到您的系统上，您
就拥有了该项目的完整历史，包括其所有的标签、提交和分支。然而，在数百个
克隆仓库四处流传的情况下，任何人如何验证他们手上linux.git 副本没有
被恶意的第三方篡改？

或者，如果在内核中发现了恶意代码，而提交中的“Author”行表明是您做的而您相当确定自己 `nothing to do with it`_（与此毫无关系）
为了解决这两个问题，Git 引入PGP 集成。签名的标签通过保证其内与创建该标签的开发者工作站上的内容完全一致，来证实仓库的完整性；签名的提交则让他人几乎不可能在无法访问您PGP 密钥的情况下冒充您
### 配置 git 以使用您PGP 密钥


如果您的密钥环中只有一个私钥，那么您实际上不需要做任何额外的事情，因为
它将成为您的默认密钥。然而，如果您恰好拥有多个私钥，您可以告git 使用
哪个密钥
```

    $ git config --global user.signingKey [fpr]

```

### 如何使用签名的标

```

    $ git tag -s [tagname]

```
我们的建议是始终git 标签签名，因为这允许其他开发者确保他们正在拉取的
git 仓库没有被恶意篡改
#### 如何验证签名的标

```

    $ git verify-tag [tagname]

```
如果您正从项目仓库的另一fork 拉取标签，git 应该会在您拉取的尖端
（tip）自动验证签```

    $ git pull [url] tags/sometag

```

```

    Merge tag 'sometag' of [url]

    [Tag message]

    # gpg: Signature made [...]
    # gpg: Good signature from [...]

```
如果您正在验证其他人git 标签，您将首先需要导入他们的 PGP 密钥。请参阅
下面“verify_identities”一节
#### 配置 git 始终签署带注解的标签


很有可能，如果您正在创建一个带注解的（annotated）标签，您会想要为其签名要强git 始终签署带注解的标签，您可以设置一个全局
```

    $ git config --global tag.forceSignAnnotated true

```

### 如何使用签名的提

也可以创建签名的提交，但它们Linux 内核开发中的用处有限。内核贡献工作流
依赖于发送补丁，而将提交转换为补丁并不会保留 git 的提交签名。此外，当您
在自己的仓库上基于更新的上游进行变基（rebase）时，PGP 提交签名最终会丢弃。出于这个原因，大多数内核开发者不会费心签署他们的提交，并且会忽略
他们所依赖的任何外部仓库中带有签名的提交
话虽如此，如果您的工git 树在某个 git 托管服务（kernel.orginfradead.org、ozlabs.org 或其他）上公开可用，那么建议是，即使上游开发不能直接从这种做法中获益，您也应该签署您所有的 git 提交
我们出于以下原因推荐这样做：

1. 如果未来需要进行代码取证或追踪代码来源（provenance），即使是外   维护的、带PGP 提交签名的树，对于此类目的也很有价值2. 如果您需要重新克隆本地仓库（例如，在重装系统之后），这让您可以在
   恢复工作之前验证仓库的完整性3. 如果有人需要挑选（cherry-pick）您的提交，这让他们在应用之前可以快   验证其完整性
#### 创建签名的提

要创建签名的提交，请`-S` 标志传递给 `git commit`
```

    $ git commit -S

```

#### 配置 git 始终签署提交


```

    git config --global commit.gpgSign true

```

    在开启此功能之前，请确保您已配置`gpg-agent`
### 如何使用签名的补

可以使用您的 PGP 密钥来为发送给内核开发者邮件列表的补丁签名。由于现有的
电子邮件签名机制（PGP-Mime PGP-inline）往往会造成常规代码审查任务问题，您应该使用 kernel.org 为此目的而创建的工具，它将密码学证明签名放入
邮件头中（类似于 DKIM 的方式）
- `Patatt Patch Attestation`_

#### 安装并配patatt



    如果您使B4 来发送您的补丁，patatt 已经安装并集成到您的工作流中
patatt 已经为许多发行版打包，因此请首先在那里查看。您也可以使pypi
通过 “`pip install patatt`安装它
如果您已经通过 git 配置PGP 密钥（经`user.signingKey` 配置参数），
那么 patatt 无需进一步配置。您可以通过安装
```

    patatt install-hook

```
来开始签署您的补丁。现在，您用 `git send-email` 发送的任何补丁都将自动
用您的密码学签名进行签署
#### 检patatt 签名


如果您使`b4` 来检索和应用补丁，那么它将自动尝试验证它遇到的所DKIM
patatt 签名
```

    $ b4 am 20220720205013.890942-1-broonie@kernel.org
    [...]
    Checking attestation on all messages, may take a moment...
    ---
      鉁?[PATCH v1 1/3] kselftest/arm64: Correct buffer allocation for SVE Z registers
      鉁?[PATCH v1 2/3] arm64/sve: Document our actual ABI for clearing registers on syscall
      鉁?[PATCH v1 3/3] kselftest/arm64: Enforce actual ABI for SVE syscalls
      ---
      鉁?Signed: openpgp/broonie@kernel.org
      鉁?Signed: DKIM/kernel.org

```

    patatt b4 仍处于积极开发中，您应该查看这些项目的最新文档以了解任何
    新增或更新的特性

## 如何验证内核开发者的身份


签署标签和提交是直截了当的，但一个人该如何去验证用于签署内容的密钥属真正的内核开发者，而不是恶意的冒名顶替者？

### 使用 WKD DANE 配置自动密钥检

如果您还不是已经拥有大量其他开发者公钥的人，那么您可以依靠密钥自动发和自动检索来启动您的密钥环。GnuPG 可以借助其他委托信任技术，DNSSEC TLS，来让您上手，如果从头开始建立自己的信任网令人生畏的话
```

    auto-key-locate wkd,dane,local
    auto-key-retrieve

```
DNS-Based Authentication of Named Entities（“DANE”，基于 DNS 的命名实认证）是一种在 DNS 中发布公钥并使用 DNSSEC 签名的区域来保护它们的方法Web Key Directory（“WKD”，Web 密钥目录）是使用 https 查询来达到相同目的替代方法。在使用 DANE WKD 查找公钥时，GnuPG 会在将自动检索到的公添加到本地密钥环之前，分别验DNSSEC TLS 证书
kernel.org 为所有拥kernel.org 账户的开发者发WKD。一旦您`gpg.conf`
中进行了上述更改，您就可以自动检Linus Torvalds Greg Kroah-Hartman
的密钥（如果```

    $ gpg --locate-keys torvalds@kernel.org gregkh@kernel.org

```
如果您拥kernel.org 账户，那么您应该 `add the kernel.org UID to your key`_
（将 kernel.org UID 添加到您的密钥）以使 WKD 对其他内核开发者更有用
### 信任网（WOT）vs. 首次使用信任（TOFU

PGP 包含一种称为“信任网（Web of Trust）”的信任委托机制。其核心是试取代 HTTPS/TLS 世界中集中式认证机构（Certification Authorities）的需要PGP 不是由各种软件制造商来决定谁应该是您可信的认证实体，而是将这一责任
留给每个用户
遗憾的是，极少有人理解信任网是如何工作的。虽然它仍然OpenPGP 规范重要组成部分，但 GnuPG 的近期版本（2.2 及以上）实现了一种称为“首次使信任（Trust on First Use，TOFU）”的替代机制。您可以TOFU 视为“类似于
SSH 的信任方式”。对SSH，您第一次连接到远程系统时，其密钥指纹会被记并记住。如果将来密钥发生变化，SSH 客户端会警告您并拒绝连接，迫使您决定
是否选择信任更改后的密钥。类似地，您第一次导入某人的 PGP 密钥时，它被
假定为有效。如GnuPG 在未来任何时刻遇到具有相同标识的另一个密钥，先前
导入的密钥和新密钥都将被标记为待验证，您将需要手动弄清楚要保留哪一个
我们建议您使用组合的 TOFU+PGP 信任模型（这GnuPG v2 中的新默认值）。要
设置它，请添加（或修改）
```

    trust-model tofu+pgp

```

### 使用 kernel.org 信任网仓

kernel.org 维护着一个包含开发者公钥的 git 仓库，以作为在过去几年中基本
变暗（gone mostly dark）的密钥服务器网络的替代。关于如何将该仓库设置为
您的公钥来源的完整文档可以在这里找到
- `Kernel developer PGP Keyring`_

如果您是一名内核开发者，请考虑提交您的密钥以纳入该密钥环