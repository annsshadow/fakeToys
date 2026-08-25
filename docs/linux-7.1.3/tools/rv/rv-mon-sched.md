
## rv-mon-sched

### 调度器监视器集合


:Manual section: 1

## 概要


**rv mon sched** [**OPTIONS**]

**rv mon <NESTED_MONITOR>** [**OPTIONS**]

**rv mon sched:<NESTED_MONITOR>** [**OPTIONS**]

## 描述


调度器监视器集合是多个监视器的容器，用于对调度器的行为进行建模。每个监视器描述一个调度器应当遵循的规范

作为监视器容器，它会启用所有嵌套监视器并根OPTIONS 进行设置。不过，嵌套监视器也可以独立激活，既可以通过名称，也可以通过指定 sched:，例如要仅启用监视器 tss，你可以执行以下任一命令

    # rv mon sched:tss

    # rv mon tss

有关此监视器的更多信息，请参阅内核文档：
<https://docs.kernel.org/trace/rv/monitor_sched.html>

## 选项


## 嵌套监视


可用的嵌套监视器有：
  - scpd：在禁用抢占的情况下调用 schedule
  - snep：schedule 不启用抢
  - sncid：不在禁用中断的情况下调schedule
  - snroc：在其自身上下文上设置为不可运行
  - sco：调度上下文操作
  - tss：调度时的任务切

## 另请参阅


**rv**\(1)銆?*rv-mon**\(1)

Linux 内核 **RV** 文档
<https://www.kernel.org/doc/html/latest/trace/rv/index.html>

## 作


Gabriele Monaco <gmonaco@redhat.com> 编写
