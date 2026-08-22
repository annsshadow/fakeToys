
# 容器化构


`container` 工具可用于在内核源码树中，从容器内部运行任意命令。这样做有助于跨各种平台复现构建过程，例如当某个测试机器人报告了一个需要特定版本编译器或外部测试套件才能复现的问题时。虽然熟悉容器的用户已经可以做到这一点，但在内核树中提供一个专用工具，可以通过一劳永逸地解决常见问题（例如用id 管理）来降低使用门槛。它还使得共享能够产生特定结果的精确命令行变得更加容易。主要用例很可能是内核构建，但几乎任何东西都可以运行：KUnit、checkpatch 等，只要存在合适的镜像即可


## 选项


```

  scripts/container -i IMAGE [OPTION]... CMD...

```
可用选项

## `-e, --env-file ENV_FILE`

    要在容器内加载的环境文件路径

## `-g, --gid GID`

    在容器内使用的组 id

## `-i, --image IMAGE`

    容器镜像名称（必填）

## `-r, --runtime RUNTIME`

    容器运行时名称。支持的运行时：`docker`、`podman`

    如果未指定，将使用系统中找到的第一个，即优先使Podman，否则使Docker

## `-s, --shell`

    以交互式 shell 运行容器

## `-u, --uid UID`

    在容器内使用的用id

    如果未指`-g` 选项，则该用id 也将用于id

## `-v, --verbose`

    启用详细输出

## `-h, --help`

    显示帮助信息并退出


## 用法


选择使用哪个镜像完全由用户决定，`CMD` 参数会作为要在容器内运行的任意命令行直接传入。该工具负责将源码树挂载为当前工作目录，并根据需要调整用户和id

通常由用户提供的、包含编译器工具链的容器镜像通过 `-i` 选项选择。容器运行时可以通过 `-r` 选项选择，可以是 `docker` `podman`。如果未指定，将使用系统中找到的第一个，并优先使Podman。对其他运行时的支持可能会根据用户中的流行程度在日后加入

默认情况下，命令以非交互方式运行。用户可以通过 SIGINT（Ctrl-C）中止正在运行的容器。要以带 TTY 的交互方式运行命令，可以使用 `--shell` `-s` 选项。此时信号将shell 直接接收，而不是由`container` 进程接收。要退出交互式 shell，请使用 Ctrl-D `exit`


   除容器运行时外，唯一的宿主机要求Python 3.10 或更高版本


   树外构建尚未完全支持。不过，`O=` 选项已经可以与源码树内的相对路径一起使用，以保留相互独立的构建输出。在树外构建的一种变通方法是使用 `mount --bind`，请参阅下文的示例部分


## 环境变量


环境变量不会传播到容器中，因此必须通过镜像本身定义，或通过 `-e` 选项使用环境文件来定义。在某些情况下，将它们定义在用于创建镜像Containerfile 中更有意义。例如，仅含 Clang 的编译器工具链镜像可能会定义 `LLVM=1`

本地环境文件更适合于在开发过程中添加的用户特定变量。它会按原样传递给容器运行时，因此其格
```

  INSTALL_MOD_STRIP=1
  SOME_RANDOM_TEXT=One upon a time

```
另请注意，`make` 选项仍然可以传递给命令行，因此虽然这无法完成，因为第一个参数必须是
```

  scripts/container -i docker.io/tuxmake/korg-clang LLVM=1 make  # won't work

```
```

  scripts/container -i docker.io/tuxmake/korg-clang make LLVM=1


```
用户 ID


这一领域的行为会因容器运行时而略有不同。目标是作为调用该工具的用户来运行命令。在 Podman 下，会创建一个命名空间，将当前用id 映射为容器内的另一id（默认为 1000）。在 Docker 下，虽然近期版本也可以做到这一点，但它需要守护进程中启用一项特殊特性，因此为简单起见这里并未使用。相反，容器直接使用当前用户 id 运行。在这两种情况下，这都会为以卷形式挂载的内核源码树提供相同的文件权限。唯一的区别是，在使用不带命名空间Docker 时，用户 id 可能与镜像中设置的默id 不同

假设我们使用一个设置了默认用户 id 1000 的镜像，而当前调`container` 工具的用id 1234。内核源码树由同一用户检出，因此这些文件属于用户 1234。在 Podman 下，容器将以用户 id 1000 运行，并映射id 1234，使得挂载卷中的文件在容器内看起来属id 1000。在使用不带命名空间Docker 时，容器将以用户 id 1234 运行，它可以访问卷中的文件，但无法访问用1000 的主目录。当只在内核树中运行命令时这不应成为问题，但这里值得强调，因为它可能对特殊边缘情况产生影响


   Podman Podman 后端之上运行 `docker` 命令`Docker 兼容<https://podman-desktop.io/docs/migrating-from-docker/managing-docker-compatibility>`__ 模式更为复杂，且尚未完全支持。因此，如果系统中两种运行时都可用，Podman 将优先


## 示例


TuxMake 项目`Docker Hub <https://hub.docker.com/u/tuxmake>`__ 上提供了一系列可用的预构建容器镜像。以下是最短的形式
```

  scripts/container -i docker.io/tuxmake/korg-clang -- make LLVM=1 defconfig
  scripts/container -i docker.io/tuxmake/korg-clang -- make LLVM=1 -j$(nproc)

```

   在容器内运行带选项的命令时，应当用双破折号 `--` 将其`container` 工具选项分隔开，以避免将它们混淆。不带选项的普通命令并不严
```

     scripts/container -i docker.io/tuxmake/korg-clang make mrproper

```
```

  scripts/container -i perl:slim-trixie scripts/checkpatch.pl patches/*

```
作为 TuxMake 镜像的替代，下面的示例引用了 `kernel.org` 镜像，它们基`kernel.org 编译器工具链 <https://mirrors.edge.kernel.org/pub/tools/>`__。这些镜像尚未（目前）在任何公共注册表中正式提供，但用户可以改为使用`实验性仓<https://gitlab.com/gtucker/korg-containers>`__，通过运行 ``make PREFIX=kernel.org/`` 在本地自行构建

```

  scripts/container -i kernel.org/clang -- make bzImage -j$(nproc)

```
```

  scripts/container -i kernel.org/gcc:15 -- make bzImage -j$(nproc)

```
对于树外构建，一个技巧是将目标目录绑定挂载到
```

  mkdir -p $HOME/tmp/my-kernel-build
  mkdir -p build
  sudo mount --bind $HOME/tmp/my-kernel-build build
  scripts/container -i kernel.org/gcc -- make mrproper
  scripts/container -i kernel.org/gcc -- make O=build defconfig
  scripts/container -i kernel.org/gcc -- make O=build -j$(nproc)

```
```

  scripts/container -s -i kernel.org/gcc:kunit -- \
      tools/testing/kunit/kunit.py \
          run \
          --arch=x86_64 \
          --cross_compile=x86_64-linux-

```
```

  scripts/container -si kernel.org/gcc bash

```
要构建需`kdocs` 镜像HTML 文档

```

  scripts/container -i kernel.org/kdocs make htmldocs

```