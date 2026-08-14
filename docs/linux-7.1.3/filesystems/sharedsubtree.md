
## 共享子树（Shared Subtrees）


 1) 概述（Overview）
 2) 特性（Features）
 3) 设置挂载状态（Setting mount states）
 4) 使用场景（Use-case）
 5) 详细语义（Detailed semantics）
 6) 测验（Quiz）
 7) 常见问题（FAQ）
 8) 实现（Implementation）


### 1) 概述（Overview）


考虑以下场景：

某个进程想要克隆（clone）它自己的命名空间（namespace），但仍希望访问最近挂载的 CD。共享子树语义（shared subtree semantics）提供了实现上述目标所需的机制。

它为诸如每用户命名空间（per-user-namespace）和版本化文件系统（versioned filesystem）等特性提供了必要的构建模块。

### 2) 特性（Features）


共享子树提供了四种不同类型的挂载（mount）；准确地说，是 struct vfsmount 的四种不同状态：


a) **共享挂载（shared mount）** 可以被复制到任意多个挂载点，并且所有副本始终保持完全一致。

   示例如下：


```

     # mount --make-shared /mnt

   .. note::
      mount(8) command now supports the --make-shared flag,
      so the sample 'smount' program is no longer needed and has been
      removed.

   ::

     # mount --bind /mnt /tmp

   The above command replicates the mount at /mnt to the mountpoint /tmp
   and the contents of both the mounts remain identical.

   ::

     #ls /mnt
     a b c

     #ls /tmp
     a b c

   Now let's say we mount a device at /tmp/a::

     # mount /dev/sd0  /tmp/a

     # ls /tmp/a
     t1 t2 t3

     # ls /mnt/a
     t1 t2 t3

   Note that the mount has propagated to the mount at /mnt as well.

   And the same is true even when /dev/sd0 is mounted on /mnt/a. The
   contents will be visible under /tmp/a too.


```
b) **从属挂载（slave mount）** 类似于共享挂载，但挂载（mount）和卸载（umount）事件只向它单向传播。

   所有从属挂载都有一个主挂载（master mount），而主挂载本身是一个共享挂载。

   示例如下：


```

     # mount --make-shared /mnt

   Let's bind mount /mnt to /tmp::

     # mount --bind /mnt /tmp

   the new mount at /tmp becomes a shared mount and it is a replica of
   the mount at /mnt.

   Now let's make the mount at /tmp; a slave of /mnt::

     # mount --make-slave /tmp

   let's mount /dev/sd0 on /mnt/a::

     # mount /dev/sd0 /mnt/a

     # ls /mnt/a
     t1 t2 t3

     # ls /tmp/a
     t1 t2 t3

   Note the mount event has propagated to the mount at /tmp

   However let's see what happens if we mount something on the mount at
   /tmp::

     # mount /dev/sd1 /tmp/b

     # ls /tmp/b
     s1 s2 s3

     # ls /mnt/b

   Note how the mount event has not propagated to the mount at
   /mnt


```
c) **私有挂载（private mount）** 既不转发也不接收传播。

   这是我们所熟悉的挂载类型，也是默认类型。


d) **不可绑定挂载（unbindable mount）** 顾名思义，是一种无法被绑定挂载的私有挂载。


```

     # mount --make-unbindable /mnt

   Let's try to bind mount this mount somewhere else::

     # mount --bind /mnt /tmp mount: wrong fs type, bad option, bad
     superblock on /mnt, or too many mounted file systems

   Binding a unbindable mount is a invalid operation.


```
### 3) 设置挂载状态（Setting mount states）


可以使用 mount 命令（util-linux 软件包）来设置挂载状态：


```

    mount --make-shared mountpoint
    mount --make-slave mountpoint
    mount --make-private mountpoint
    mount --make-unbindable mountpoint


```
### 4) 使用场景（Use cases）


A) 某个进程想要克隆自己的命名空间，但仍希望访问最近挂载的 CD。

   解决方案：


```

     mount --bind /cdrom /cdrom
     mount --make-shared /cdrom

   Now any process that clones off a new namespace will have a
   mount at /cdrom which is a replica of the same mount in the
   parent namespace.

   So when a CD is inserted and mounted at /cdrom that mount gets
   propagated to the other mount at /cdrom in all the other clone
   namespaces.

```
B) 某个进程希望自己的挂载对其他任何进程不可见，但仍能看到系统的其他挂载。

   解决方案：

   首先，管理员可以将整个挂载树标记为


```

     mount --make-rshared /

   A new process can clone off a new namespace. And mark some part
   of its namespace as slave::

     mount --make-rslave /myprivatetree

   Hence forth any mounts within the /myprivatetree done by the
   process will not show up in any other namespace. However mounts
   done in the parent namespace under /myprivatetree still shows
   up in the process's namespace.


```
除了上述语义之外，该特性还为解决以下问题提供了构建模块：

C) 每用户命名空间（Per-user namespace）

   上述语义提供了一种跨命名空间共享挂载的方式。但命名空间是与进程相关联的。如果命名空间被实现为一流对象，并提供用户 API 来将命名空间与用户 ID 关联/解除关联，那么每个用户都可以拥有他/她自己的命名空间，并根据其需求进行定制。这需要在 PAM 中提供支持。

D) 版本化文件（Versioned files）

   如果整个挂载树在多个位置可见，那么底层的版本化文件系统可以根据用于访问该文件的路径，返回该文件的不同版本。


```

       mount --make-shared /
       mount --rbind / /view/v1
       mount --rbind / /view/v2
       mount --rbind / /view/v3
       mount --rbind / /view/v4

    and if /usr has a versioning filesystem mounted, then that
    mount appears at /view/v1/usr, /view/v2/usr, /view/v3/usr and
    /view/v4/usr too

    A user can request v3 version of the file /usr/fs/namespace.c
    by accessing /view/v3/usr/fs/namespace.c . The underlying
    versioning filesystem can then decipher that v3 version of the
    filesystem is being requested and return the corresponding
    inode.

```
### 5) 详细语义（Detailed semantics）

本节解释了 bind、rbind、move、mount、umount 以及克隆命名空间（clone-namespace）操作的详细语义。

   本文档中，单词 'vfsmount' 和名词 'mount' 被用来指代同一个概念。

a) 挂载状态（Mount states）

   **传播事件（propagation event）** 定义为在某个 vfsmount 上产生、并导致其他 vfsmount 上发生挂载或卸载动作的事件。

   **对等组（peer group）** 定义为一组相互传播事件的 vfsmount。

   一个给定的挂载可以处于以下状态之一：

   (1) 共享挂载（Shared mounts）

       **共享挂载（shared mount）** 定义为属于某个对等组的 vfsmount。


```

         mount --make-shared /mnt
         mount --bind /mnt /tmp

       The mount at /mnt and that at /tmp are both shared and belong
       to the same peer group. Anything mounted or unmounted under
       /mnt or /tmp reflect in all the other mounts of its peer
       group.


   (2) Slave mounts

       A **slave mount** is defined as a vfsmount that receives
       propagation events and does not forward propagation events.

       A slave mount as the name implies has a master mount from which
       mount/unmount events are received. Events do not propagate from
       the slave mount to the master.  Only a shared mount can be made
       a slave by executing the following command::

         mount --make-slave mount

       A shared mount that is made as a slave is no more shared unless
       modified to become shared.

   (3) Shared and Slave

       A vfsmount can be both **shared** as well as **slave**.  This state
       indicates that the mount is a slave of some vfsmount, and
       has its own peer group too.  This vfsmount receives propagation
       events from its master vfsmount, and also forwards propagation
       events to its 'peer group' and to its slave vfsmounts.

       Strictly speaking, the vfsmount is shared having its own
       peer group, and this peer-group is a slave of some other
       peer group.

       Only a slave vfsmount can be made as 'shared and slave' by
       either executing the following command::

         mount --make-shared mount

       or by moving the slave vfsmount under a shared vfsmount.

   (4) Private mount

       A **private mount** is defined as vfsmount that does not
       receive or forward any propagation events.

   (5) Unbindable mount

       A **unbindable mount** is defined as vfsmount that does not
       receive or forward any propagation events and cannot
       be bind mounted.


       State diagram:

       The state diagram below explains the state transition of a mount,
       in response to various commands::

            -----------------------------------------------------------------------
            |             |make-shared |  make-slave  | make-private |make-unbindab|
            --------------|------------|--------------|--------------|-------------|
            |shared       |shared      |*slave/private|   private    | unbindable  |
            |             |            |              |              |             |
            |-------------|------------|--------------|--------------|-------------|
            |slave        |shared      | **slave      |    private   | unbindable  |
            |             |and slave   |              |              |             |
            |-------------|------------|--------------|--------------|-------------|
            |shared       |shared      | slave        |    private   | unbindable  |
            |and slave    |and slave   |              |              |             |
            |-------------|------------|--------------|--------------|-------------|
            |private      |shared      |  **private   |    private   | unbindable  |
            |-------------|------------|--------------|--------------|-------------|
            |unbindable   |shared      |**unbindable  |    private   | unbindable  |
            ------------------------------------------------------------------------

            * if the shared mount is the only mount in its peer group, making it
            slave, makes it private automatically. Note that there is no master to
            which it can be slaved to.

            ** slaving a non-shared mount has no effect on the mount.

       Apart from the commands listed below, the 'move' operation also changes
       the state of a mount depending on type of the destination mount. Its
       explained in section 5d.

```
b) 绑定语义（Bind semantics）


```

     mount --bind A/a  B/b

   where 'A' is the source mount, 'a' is the dentry in the mount 'A', 'B'
   is the destination mount and 'b' is the dentry in the destination mount.

   The outcome depends on the type of mount of 'A' and 'B'. The table
   below contains quick reference::

            --------------------------------------------------------------------------
            |         BIND MOUNT OPERATION                                           |
            |************************************************************************|
            |source(A)->| shared      |       private  |       slave    | unbindable |
            | dest(B)  |              |                |                |            |
            |   |      |              |                |                |            |
            |   v      |              |                |                |            |
            |************************************************************************|
            |  shared  | shared       |     shared     | shared & slave |  invalid   |
            |          |              |                |                |            |
            |non-shared| shared       |      private   |      slave     |  invalid   |
            **************************************************************************

   Details:

   1. 'A' is a shared mount and 'B' is a shared mount. A new mount 'C'
      which is clone of 'A', is created. Its root dentry is 'a' . 'C' is
      mounted on mount 'B' at dentry 'b'. Also new mount 'C1', 'C2', 'C3' ...
      are created and mounted at the dentry 'b' on all mounts where 'B'
      propagates to. A new propagation tree containing 'C1',..,'Cn' is
      created. This propagation tree is identical to the propagation tree of
      'B'.  And finally the peer-group of 'C' is merged with the peer group
      of 'A'.

   2. 'A' is a private mount and 'B' is a shared mount. A new mount 'C'
      which is clone of 'A', is created. Its root dentry is 'a'. 'C' is
      mounted on mount 'B' at dentry 'b'. Also new mount 'C1', 'C2', 'C3' ...
      are created and mounted at the dentry 'b' on all mounts where 'B'
      propagates to. A new propagation tree is set containing all new mounts
      'C', 'C1', .., 'Cn' with exactly the same configuration as the
      propagation tree for 'B'.

   3. 'A' is a slave mount of mount 'Z' and 'B' is a shared mount. A new
      mount 'C' which is clone of 'A', is created. Its root dentry is 'a' .
      'C' is mounted on mount 'B' at dentry 'b'. Also new mounts 'C1', 'C2',
      'C3' ... are created and mounted at the dentry 'b' on all mounts where
      'B' propagates to. A new propagation tree containing the new mounts
      'C','C1',..  'Cn' is created. This propagation tree is identical to the
      propagation tree for 'B'. And finally the mount 'C' and its peer group
      is made the slave of mount 'Z'.  In other words, mount 'C' is in the
      state 'slave and shared'.

   4. 'A' is a unbindable mount and 'B' is a shared mount. This is a
      invalid operation.

   5. 'A' is a private mount and 'B' is a non-shared(private or slave or
      unbindable) mount. A new mount 'C' which is clone of 'A', is created.
      Its root dentry is 'a'. 'C' is mounted on mount 'B' at dentry 'b'.

   6. 'A' is a shared mount and 'B' is a non-shared mount. A new mount 'C'
      which is a clone of 'A' is created. Its root dentry is 'a'. 'C' is
      mounted on mount 'B' at dentry 'b'.  'C' is made a member of the
      peer-group of 'A'.

   7. 'A' is a slave mount of mount 'Z' and 'B' is a non-shared mount. A
      new mount 'C' which is a clone of 'A' is created. Its root dentry is
      'a'.  'C' is mounted on mount 'B' at dentry 'b'. Also 'C' is set as a
      slave mount of 'Z'. In other words 'A' and 'C' are both slave mounts of
      'Z'.  All mount/unmount events on 'Z' propagates to 'A' and 'C'. But
      mount/unmount on 'A' do not propagate anywhere else. Similarly
      mount/unmount on 'C' do not propagate anywhere else.

   8. 'A' is a unbindable mount and 'B' is a non-shared mount. This is a
      invalid operation. A unbindable mount cannot be bind mounted.

```
c) 递归绑定语义（Rbind semantics）

   rbind 与 bind 相同。bind 复制指定的挂载。rbind 复制属于指定挂载的树中的所有挂载。rbind 挂载就是对树中所有挂载应用的 bind 挂载。

   如果被 rbind 的源树中包含一些不可绑定挂载，那么这些不可绑定挂载之下的子树会在新位置被剪除。

   例如：


```

                A
              /   \
              B   C
             / \ / \
             D E F G

   Let's say all the mount except the mount C in the tree are
   of a type other than unbindable.

   If this tree is rbound to say Z

   We will have the following tree at the new location::

                Z
                |
                A'
               /
              B'                Note how the tree under C is pruned
             / \                in the new location.
            D' E'



```
d) 移动语义（Move semantics）


```

     mount --move A  B/b

   where 'A' is the source mount, 'B' is the destination mount and 'b' is
   the dentry in the destination mount.

   The outcome depends on the type of the mount of 'A' and 'B'. The table
   below is a quick reference::

            ---------------------------------------------------------------------------
            |                   MOVE MOUNT OPERATION                                 |
            |**************************************************************************
            | source(A)->| shared      |       private  |       slave    | unbindable |
            | dest(B)  |               |                |                |            |
            |   |      |               |                |                |            |
            |   v      |               |                |                |            |
            |**************************************************************************
            |  shared  | shared        |     shared     |shared and slave|  invalid   |
            |          |               |                |                |            |
            |non-shared| shared        |      private   |    slave       | unbindable |
            ***************************************************************************

   .. Note:: moving a mount residing under a shared mount is invalid.

   Details follow:

   1. 'A' is a shared mount and 'B' is a shared mount.  The mount 'A' is
      mounted on mount 'B' at dentry 'b'.  Also new mounts 'A1', 'A2'...'An'
      are created and mounted at dentry 'b' on all mounts that receive
      propagation from mount 'B'. A new propagation tree is created in the
      exact same configuration as that of 'B'. This new propagation tree
      contains all the new mounts 'A1', 'A2'...  'An'.  And this new
      propagation tree is appended to the already existing propagation tree
      of 'A'.

   2. 'A' is a private mount and 'B' is a shared mount. The mount 'A' is
      mounted on mount 'B' at dentry 'b'. Also new mount 'A1', 'A2'... 'An'
      are created and mounted at dentry 'b' on all mounts that receive
      propagation from mount 'B'. The mount 'A' becomes a shared mount and a
      propagation tree is created which is identical to that of
      'B'. This new propagation tree contains all the new mounts 'A1',
      'A2'...  'An'.

   3. 'A' is a slave mount of mount 'Z' and 'B' is a shared mount.  The
      mount 'A' is mounted on mount 'B' at dentry 'b'.  Also new mounts 'A1',
      'A2'... 'An' are created and mounted at dentry 'b' on all mounts that
      receive propagation from mount 'B'. A new propagation tree is created
      in the exact same configuration as that of 'B'. This new propagation
      tree contains all the new mounts 'A1', 'A2'...  'An'.  And this new
      propagation tree is appended to the already existing propagation tree of
      'A'.  Mount 'A' continues to be the slave mount of 'Z' but it also
      becomes 'shared'.

   4. 'A' is a unbindable mount and 'B' is a shared mount. The operation
      is invalid. Because mounting anything on the shared mount 'B' can
      create new mounts that get mounted on the mounts that receive
      propagation from 'B'.  And since the mount 'A' is unbindable, cloning
      it to mount at other mountpoints is not possible.

   5. 'A' is a private mount and 'B' is a non-shared(private or slave or
      unbindable) mount. The mount 'A' is mounted on mount 'B' at dentry 'b'.

   6. 'A' is a shared mount and 'B' is a non-shared mount.  The mount 'A'
      is mounted on mount 'B' at dentry 'b'.  Mount 'A' continues to be a
      shared mount.

   7. 'A' is a slave mount of mount 'Z' and 'B' is a non-shared mount.
      The mount 'A' is mounted on mount 'B' at dentry 'b'.  Mount 'A'
      continues to be a slave mount of mount 'Z'.

   8. 'A' is a unbindable mount and 'B' is a non-shared mount. The mount
      'A' is mounted on mount 'B' at dentry 'b'. Mount 'A' continues to be a
      unbindable mount.

```
e) 挂载语义（Mount semantics）


```

     mount device  B/b

   'B' is the destination mount and 'b' is the dentry in the destination
   mount.

   The above operation is the same as bind operation with the exception
   that the source mount is always a private mount.


```
f) 卸载语义（Unmount semantics）


```

     umount A

   where 'A' is a mount mounted on mount 'B' at dentry 'b'.

   If mount 'B' is shared, then all most-recently-mounted mounts at dentry
   'b' on mounts that receive propagation from mount 'B' and does not have
   sub-mounts within them are unmounted.

   Example: Let's say 'B1', 'B2', 'B3' are shared mounts that propagate to
   each other.

   let's say 'A1', 'A2', 'A3' are first mounted at dentry 'b' on mount
   'B1', 'B2' and 'B3' respectively.

   let's say 'C1', 'C2', 'C3' are next mounted at the same dentry 'b' on
   mount 'B1', 'B2' and 'B3' respectively.

   if 'C1' is unmounted, all the mounts that are most-recently-mounted on
   'B1' and on the mounts that 'B1' propagates-to are unmounted.

   'B1' propagates to 'B2' and 'B3'. And the most recently mounted mount
   on 'B2' at dentry 'b' is 'C2', and that of mount 'B3' is 'C3'.

   So all 'C1', 'C2' and 'C3' should be unmounted.

   If any of 'C2' or 'C3' has some child mounts, then that mount is not
   unmounted, but all other mounts are unmounted. However if 'C1' is told
   to be unmounted and 'C1' has some sub-mounts, the umount operation is
   failed entirely.

```
g) 克隆命名空间（Clone Namespace）

   一个克隆出的命名空间包含与父命名空间相同的所有挂载。

   假设 'A' 和 'B' 分别是父命名空间和子命名空间中对应的挂载。

   如果 'A' 是共享的，那么 'B' 也是共享的，并且 'A' 和 'B' 相互传播。

   如果 'A' 是挂载 'Z' 的从属挂载，那么 'B' 也是 'Z' 的从属挂载。

   如果 'A' 是私有挂载，那么 'B' 也是私有挂载。

   如果 'A' 是不可绑定挂载，那么 'B' 也是不可绑定挂载。


### 6) 测验（Quiz）


A. 以下命令序列的结果是什么？


```

       mount --bind /mnt /mnt
       mount --make-shared /mnt
       mount --bind /mnt /tmp
       mount --move /tmp /mnt/1

   what should be the contents of /mnt /mnt/1 /mnt/1/1 should be?
   Should they all be identical? or should /mnt and /mnt/1 be
   identical only?


```
B. 以下命令序列的结果是什么？


```

       mount --make-rshared /
       mkdir -p /v/1
       mount --rbind / /v/1

   what should be the content of /v/1/v/1 be?


```
C. 以下命令序列的结果是什么？


```

       mount --bind /mnt /mnt
       mount --make-shared /mnt
       mkdir -p /mnt/1/2/3 /mnt/1/test
       mount --bind /mnt/1 /tmp
       mount --make-slave /mnt
       mount --make-shared /mnt
       mount --bind /mnt/1/2 /tmp1
       mount --make-slave /mnt

   At this point we have the first mount at /tmp and
   its root dentry is 1. Let's call this mount 'A'
   And then we have a second mount at /tmp1 with root
   dentry 2. Let's call this mount 'B'
   Next we have a third mount at /mnt with root dentry
   mnt. Let's call this mount 'C'

   'B' is the slave of 'A' and 'C' is a slave of 'B'
   A -> B -> C

   at this point if we execute the following command::

     mount --bind /bin /tmp/test

   The mount is attempted on 'A'

   will the mount propagate to 'B' and 'C' ?

   what would be the contents of
   /mnt/1/test be?

```
### 7) 常见问题（FAQ）


1. 为什么需要绑定挂载？它与符号链接有何不同？

   符号链接在目标挂载被卸载或移动时可能会失效。而绑定挂载即使其他挂载被卸载或移动也依然存在。

2. 为什么不能只用 exportfs 来实现共享子树？

   exportfs 是一种实现共享子树部分功能的高开销方式。我无法想象出一种能用 exportfs 实现从属挂载语义的方法。

3. 为什么需要不可绑定挂载？

   假设我们想在同一子树内的多个位置复制挂载树。

   如果在同一子树内将一棵树 rbind 挂载 'n' 次，所创建的挂载数量是 'n' 的指数函数。使用不可绑定挂载有助于剪除不需要的绑定挂载。下面是一个例子。

   step 1:
      假设根树只有两个目录


```

                                    root
                                   /    \
                                  tmp    usr

      And we want to replicate the tree at multiple
      mountpoints under /root/tmp

   step 2:
      ::


                        mount --make-shared /root

                        mkdir -p /tmp/m1

                        mount --rbind /root /tmp/m1

      the new tree now looks like this::

                                    root
                                   /    \
                                 tmp    usr
                                /
                               m1
                              /  \
                             tmp  usr
                             /
                            m1

      it has two vfsmounts

   step 3:
      ::

                            mkdir -p /tmp/m2
                            mount --rbind /root /tmp/m2

      the new tree now looks like this::

                                      root
                                     /    \
                                   tmp     usr
                                  /    \
                                m1       m2
                               / \       /  \
                             tmp  usr   tmp  usr
                             / \          /
                            m1  m2      m1
                                / \     /  \
                              tmp usr  tmp   usr
                              /        / \
                             m1       m1  m2
                            /  \
                          tmp   usr
                          /  \
                         m1   m2

                    it has 6 vfsmounts

   step 4:
      ::

                          mkdir -p /tmp/m3
                          mount --rbind /root /tmp/m3

      I won't draw the tree..but it has 24 vfsmounts


   at step i the number of vfsmounts is V[i] = i*V[i-1].
   This is an exponential function. And this tree has way more
   mounts than what we really needed in the first place.

   One could use a series of umount at each step to prune
   out the unneeded mounts. But there is a better solution.
   Unclonable mounts come in handy here.

   step 1:
      let's say the root tree has just two directories with
      one vfsmount::

                                    root
                                   /    \
                                  tmp    usr

         How do we set up the same tree at multiple locations under
         /root/tmp

   step 2:
      ::


                        mount --bind /root/tmp /root/tmp

                        mount --make-rshared /root
                        mount --make-unbindable /root/tmp

                        mkdir -p /tmp/m1

                        mount --rbind /root /tmp/m1

      the new tree now looks like this::

                                    root
                                   /    \
                                 tmp    usr
                                /
                               m1
                              /  \
                             tmp  usr

   step 3:
      ::

                            mkdir -p /tmp/m2
                            mount --rbind /root /tmp/m2

      the new tree now looks like this::

                                    root
                                   /    \
                                 tmp    usr
                                /   \
                               m1     m2
                              /  \     / \
                             tmp  usr tmp usr

   step 4:
      ::

                            mkdir -p /tmp/m3
                            mount --rbind /root /tmp/m3

      the new tree now looks like this::

                                          root
                                      /           \
                                     tmp           usr
                                 /    \    \
                               m1     m2     m3
                              /  \     / \    /  \
                             tmp  usr tmp usr tmp usr

```
### 8) 实现（Implementation）


A) 数据结构（Datastructure）

   为 struct vfsmount 引入了几条新字段：

   ->mnt_share
           将所有从这个 vfsmount 发送/接收传播事件的挂载链接在一起。

   ->mnt_slave_list
           链接这个 vfsmount 传播到的所有挂载。

   ->mnt_slave
           将这个 vfsmount 的主（master）vfsmount 传播到的所有从属链接在一起。

   ->mnt_master
           指向这个 vfsmount 从中接收传播的主 vfsmount。

   ->mnt_flags
           增加了两个标志位，用于指示 vfsmount 的传播状态。MNT_SHARE 表示该 vfsmount 是一个共享 vfsmount。MNT_UNCLONABLE 表示该 vfsmount 不能被复制。

   一个对等组中的所有共享 vfsmount 通过 ->mnt_share 形成一个循环链表。

   所有具有相同 ->mnt_master 的 vfsmount 形成一个循环链表，该链表锚定在 ->mnt_master->mnt_slave_list 中，并通过 ->mnt_slave 串联。

   ->mnt_master 可以指向主对等组任意（且可能不同）的成员。要找到一个对等组的所有直接从属，需要遍历其所有成员的 _所有_ ->mnt_slave_list。从概念上讲它是一个单一的集合——分布到各个链表上并不影响传播，也不影响操作对传播树的修改方式。

   一个对等组中的所有 vfsmount 拥有相同的 ->mnt_master。如果它非 NULL，它们就形成一个连续的（有序的）从属链表段。

   一个示例传播树如下图所示。


```
      Though it looks like a forest, if we consider all the shared
      mounts as a conceptual entity called 'pnode', it becomes a tree.

   ::


                        A <--> B <--> C <---> D
                       /|\            /|      |\
                      / F G          J K      H I
                     /
                    E<-->K
                        /|\
                       M L N

   In the above figure  A,B,C and D all are shared and propagate to each
   other.   'A' has got 3 slave mounts 'E' 'F' and 'G' 'C' has got 2 slave
   mounts 'J' and 'K'  and  'D' has got two slave mounts 'H' and 'I'.
   'E' is also shared with 'K' and they propagate to each other.  And
   'K' has 3 slaves 'M', 'L' and 'N'

   A's ->mnt_share links with the ->mnt_share of 'B' 'C' and 'D'

   A's ->mnt_slave_list links with ->mnt_slave of 'E', 'K', 'F' and 'G'

   E's ->mnt_share links with ->mnt_share of K

   'E', 'K', 'F', 'G' have their ->mnt_master point to struct vfsmount of 'A'

   'M', 'L', 'N' have their ->mnt_master point to struct vfsmount of 'K'

   K's ->mnt_slave_list links with ->mnt_slave of 'M', 'L' and 'N'

   C's ->mnt_slave_list links with ->mnt_slave of 'J' and 'K'

   J and K's ->mnt_master points to struct vfsmount of C

   and finally D's ->mnt_slave_list links with ->mnt_slave of 'H' and 'I'

   'H' and 'I' have their ->mnt_master pointing to struct vfsmount of 'D'.


   NOTE: The propagation tree is orthogonal to the mount tree.

```
B) 加锁（Locking）：

   ->mnt_share、->mnt_slave、->mnt_slave_list、->mnt_master 由 namespace_sem 保护（修改时独占，读取时共享）。

   通常我们通过 vfsmount_lock 来串行化 ->mnt_flags 的修改。有两个例外：do_add_mount() 和 clone_mnt()。前者修改一个尚未在任何共享数据结构中可见的 vfsmount。后者持有 namespace_sem，且对 vfsmount 的唯一引用都位于不持有 namespace_sem 就无法遍历的链表中。

C) 算法（Algorithm）：

   实现的核心在于 rbind/move 操作。

   总体算法将该操作分解为 3 个阶段：（参见 attach_recursive_mnt() 和 propagate_mnt()）

   1. 准备阶段（Prepare phase）。

      对于源树中的每个挂载：

      a) 创建所需数量的挂载树，以附加到从目标挂载接收传播的所有挂载上。
      b) 不要将任何树附加到其目标上。但要记录其 ->mnt_parent 和 ->mnt_mountpoint。
      c) 将所有新挂载链接起来，形成一个与目标挂载的传播树完全相同的传播树。

      如果此阶段成功，应当创建 'n' 个新的传播树，其中 'n' 是源树中挂载的数量。进入提交阶段。

      同时应当创建 'm' 个新的挂载树，其中 'm' 是目标挂载传播到的挂载数量。

      如果任何内存分配失败，进入中止阶段。

   2. 提交阶段（Commit phase）。

      将每个挂载树附加到其对应的目标挂载上。

   3. 中止阶段（Abort phase）。

      删除所有新创建的树。


```
      all the propagation related functionality resides in the file pnode.c


```
------------------------------------------------------------------------

version 0.1  (created the initial document, Ram Pai linuxram@us.ibm.com)

version 0.2  (Incorporated comments from Al Viro)

