## 关于 kobjects、ksets ktypes 那些你从不想了解的事


:Author: Greg Kroah-Hartman <gregkh@linuxfoundation.org>
:Last updated: December 19, 2007

基于 Jon Corbet lwn.net 所写、发表于 2003 10 1 日、位https://lwn.net/Articles/51437/ 的原创文章
理解驱动模型 —以及构建于其上的 kobject 抽象 —的困难之一在于没有显而易见的起点。处kobject 需要理解几种不同的类型，而它们彼此相互引用。为了让事情更简单，我们将采用多次递进的方式，从模糊的术语开始，并逐步补充细节。为此，下面先给出我们将用到的一些术语的快速定义
 - kobject struct kobject 类型的对象。kobject 具有一个名称和引用计数。kobject 还有一个父指针（允许将对象排列成层级结构）、一个特定类型，以及通常sysfs 虚拟文件系统中的一种表示
   kobject 本身一般并不引人关注；相反，它们通常嵌入在包含代码真正感兴趣内容的其他结构中
   任何结构体都**绝不*嵌入多于一kobject。如果有，该对象的引用计数必定会混乱且不正确，你的代码就会有 bug。所以不要这样做
 - ktype 是嵌入了 kobject 的对象的类型。每个嵌入了 kobject 的结构体都需要一个对应的 ktype。ktype 控制kobject 被创建和销毁时发生的事情
 - kset 是一kobject。这kobject 可以是相同的 ktype，也可以属于不同ktype。kset kobject 集合的基本容器类型。kset 包含它们自己kobject，但你可以放心地忽略这一实现细节，因kset 核心代码会自动处理这kobject
   当你看到一个填满其他目录的 sysfs 目录时，通常其中每个目录都对应于同一kset 中的一kobject
我们将了解如何创建和操作所有这些类型。我们将采用自底向上的方法，因此先回kobject

## 嵌入 kobject


内核代码很少会创建一个独立的 kobject，唯一的重大例外将在下文说明。相反，kobject 被用来控制对更大的、特定领域对象的访问。为此，kobject 会嵌入在其他结构中。如果你习惯于用面向对象的术语思考问题，kobject 可被视为一个顶层的抽象类，其他类从它派生。kobject 实现了一组本身用处不大、但在其他对象中很好用的能力。C 语言不允许直接表达继承，因此必须使用其他技术——例如结构体嵌入
（顺便说一句，对于熟悉内核链表实现的人，这类似"list_head" 结构体本身很少单独有用，但总是会嵌入在感兴趣的大对象中。）

因此，例`drivers/uio/uio.c` 中的 UIO 代码有一个结构体
```

    struct uio_map {
            struct kobject kobj;
            struct uio_mem *mem;
    };

```
如果你有一struct uio_map 结构体，找到其嵌入的 kobject 只需使用 kobj 成员。不过，处理 kobject 的代码通常会遇到相反的问题：给定一struct kobject 指针，指向包含它的结构体的指针是什么？你必须避免那些取巧手段（例如假设 kobject 位于结构体的开头）
```

    container_of(ptr, type, member)

```
其中
  - `ptr` 是指向嵌入的 kobject 的指针，
  - `type` 是包含该结构体的类型，并  - `member` `pointer` 所指向的结构体字段的名称
container_of() 的返回值是指向相应容器类型的指针。因此，例如，一个指向嵌** struct uio_map 中的 struct kobject 的指`kp` 可以被转换为指向```

    struct uio_map *u_map = container_of(kp, struct uio_map, kobj);

```
为了方便，程序员常常定义一个简单的宏，用于kobject 指针**反向转换**回包含它的类型。在下面这种情况中正是如```

    struct uio_map {
            struct kobject kobj;
            struct uio_mem *mem;
    };

    #define to_map(map) container_of(map, struct uio_map, kobj)

```
其中宏参"map" 是指struct uio_map struct kobject 的指针，位于
```

    struct uio_map *map = to_map(kobj);


```
## kobject 的初始化


创建 kobject 的代码当然必须初始化该对象。一```

    void kobject_init(struct kobject *kobj, const struct kobj_type *ktype);

```
要使 kobject 被正确创建，ktype 是必需的，因为每个 kobject 都必须有一个关联的 kobj_type。调kobject_init() 之后，要
```

    int kobject_add(struct kobject *kobj, struct kobject *parent,
                    const char *fmt, ...);

```
这会正确设置 kobject 的父对象及其名称。如kobject 要关联到特定kset，则必须在调kobject_add() 之前赋kobj->kset。如kset kobject 关联，那么在调用 kobject_add() 时可kobject 的父对象设为 NULL，此kobject 的父对象将是 kset 自身
由于 kobject 的名称是在它被加入内核时设置的，因此绝不应直接操kobject 的名称。如果你必须更改
```

    int kobject_rename(struct kobject *kobj, const char *new_name);

```
   kobject_rename() 不执行任何加锁，也没有关于哪些名称有效的确切概念，因此调用者必须自己提供健全性检查和串行化
有一个名kobject_set_name() 的函数，但那属于历史遗留垃圾，正在被移除。如果你的代码需要调用这个函数，那是不正确的，需要修复
要正确访kobject 的名称，使用函数
```

    const char *kobject_name(const struct kobject * kobj);

```
有一个辅助函数可同时初始kobject 并将其加```

    int kobject_init_and_add(struct kobject *kobj, const struct kobj_type *ktype,
                             struct kobject *parent, const char *fmt, ...);

```
   其参数与前面描述kobject_init() kobject_add() 函数相同

## Uevents（用户态事件）


kobject kobject 核心注册之后，你需要向外界宣告它已被创建。这可以通过
```

    int kobject_uevent(struct kobject *kobj, enum kobject_action action);

```
   kobject 首次加入内核时使**KOBJ_ADD** 动作。这应当仅在 kobject 的任何属性或子对象都已正确初始化之后进行，因为该调用发生时用户空间会立即开始查找它们
   kobject 从内核移除时（如何操作的细节见下文）*KOBJ_REMOVE** uevent 会由 kobject 核心自动创建，因此调用者无需操心手动去做

## 引用计数


kobject 的关键功能之一是充当其所嵌入对象的引用计数器。只要对该对象的引用存在，该对象（及其支撑代码）就必须继续存在```

    struct kobject *kobject_get(struct kobject *kobj);
    void kobject_put(struct kobject *kobj);

```
   成功调用 kobject_get() 会递增 kobject 的引用计数，并返回指向该 kobject 的指针
   当释放一个引用时，调kobject_put() 会递减引用计数，并可能释放该对象。注kobject_init() 将引用计数设1，因此设kobject 的代码最终需要调用一kobject_put() 来释放该引用
由于 kobject 是动态的，它们绝不能静态声明或在栈上声明，而必须始终动态分配。未来的内核版本将包含对静态创建的 kobject 的运行时检查，并向开发者警告这种不当用法
如果你的全部需求只是为你的结构体提供引用计数器，请改用 struct kref；使kobject 则过于重量级。有关如何使struct kref 的更多信息，请参Linux 内核源代码树中的 Documentation/core-api/kref.rst 文件

## 创建“简单”的 kobject


有时开发者想要的只是sysfs 层级中创建一个简单目录的方法，而不必去处理 kset、show store 函数以及其他细节这一整套复杂性。这是应当创建单kobject 的唯一例外情况。要创建这样一```

    struct kobject *kobject_create_and_add(const char *name, struct kobject *parent);

```
该函数将创建一kobject，并将其置于 sysfs 中指定父 kobject 下方的位置。要创建简单属```

    int sysfs_create_file(struct kobject *kobj, const struct attribute *attr);

```
```

    int sysfs_create_group(struct kobject *kobj, const struct attribute_group *grp);

```
   这里使用的两类属性，配合kobject_create_and_add() 创建kobject，都可以kobj_attribute 类型，因此无需创建特殊的自定义属性
有关简kobject 和属性的实现，请参见示例模块 `samples/kobject/kobject-example.c`

## ktype 与释放方

讨论中仍然缺少的一个重要问题是：当 kobject 的引用计数降为零时会发生什么。创kobject 的代码通常不知道这何时会发生；如果知道，一开始也就没多大必要使用 kobject 了。当引入 sysfs 后，即便是可预测的对象生命周期也会变得更复杂，因为内核的其他部分可以获取系统中任何已注册 kobject 的引用
最终结果是，受 kobject 保护的结构体在其引用计数归零之前不能被释放。引用计数不受创kobject 的代码的直接控制。因此，每当其某kobject 的最后一个引用消失时，该代码必须以异步方式得到通知
一旦你通过 kobject_add() 注册kobject，就绝不能用 kfree() 直接释放它。唯一安全的方式是使用 kobject_put()。良好实践是始终kobject_init() 之后使用 kobject_put()，以避免错误悄悄混入
这种通知通过 kobject release() 方法完成。通常
```

    void my_object_release(struct kobject *kobj)
    {
            struct my_object *mine = container_of(kobj, struct my_object, kobj);

            /* Perform any additional cleanup on this object, then... */
            kfree(mine);
    }

```
   有一个要点怎么强调都不为过：每kobject 都必须有一release() 方法，并kobject 必须保持存在（处于一致状态）直到该方法被调用。如果不满足这些约束，代码就是有缺陷的。注意，如果你忘记提release() 方法，内核会警告你。不要试图通过提供一个“空”的 release 函数来消除这个警告
   如果你的清理函数只需调用 kfree()，那么你必须创建一个包装函数，使用 container_of() 向上转型为正确的类型（如上面的例子所示），然后对整个结构体调kfree()
   注意，kobject 的名称在 release 函数中可用，但绝不应在此回调中更改它。否kobject 核心中会出现内存泄漏，这会让人不快
有趣的是，release() 方法并不存储kobject 自身中，而是ktype 关联。因此让我们引入 struct
```

    struct kobj_type {
            void (*release)(struct kobject *kobj);
            const struct sysfs_ops *sysfs_ops;
            const struct attribute_group **default_groups;
            const struct kobj_ns_type_operations *(*child_ns_type)(struct kobject *kobj);
            const void *(*namespace)(struct kobject *kobj);
            void (*get_ownership)(struct kobject *kobj, kuid_t *uid, kgid_t *gid);
    };

```
   该结构体用于描述一种特定类型的 kobject（或者更准确地说，是包含它的对象）。每kobject 都需要有一个关联的 kobj_type 结构体；在调kobject_init() kobject_init_and_add() 时必须指定指向该结构体的指针
struct kobj_type 中的 release 字段当然是指向此kobject release() 方法的指针。另外两个字段（sysfs_ops default_groups）控制此类对象在 sysfs 中的表示方式；这超出了本文档的范围
default_groups 指针是一个默认属性列表，会为注册到该 ktype 的任kobject 自动创建

## kset


kset 仅仅是一组希望彼此关联的 kobject 的集合。并不要求它们属于相同的 ktype，但如果不是，则要非常小心
kset 提供以下功能
 - 它充当一个容纳一组对象的袋子。内核可以用 kset 来跟踪“所有块设备”或“所PCI 设备驱动”
 - kset 也是 sysfs 中的一个子目录，关联的 kobject 可以出现其中。每kset 包含一kobject，该 kobject 可被设置为其kobject 的父对象；sysfs 层级中的顶级目录就是以这种方式构建的
 - kset 可以支持 kobject 的“热插拔”，并影uevent 事件如何上报给用户空间
用面向对象的术语说，“kset是顶层的容器类；kset 包含它们自己kobject，但那个 kobject kset 代码管理，不应被任何其他用户操纵
kset 将其子对象保存在一个标准的 kernel 链表中。kobject 通过它们kset 字段指回其所属的 kset。在几乎所有情况下，属于某kset kobject 在其父对象中拥有kset（严格地说，是其嵌入kobject）
由于 kset 内部包含一kobject，它应始终被动态创建，而绝不能静态声明或在栈上声明。要创建一个新```

  struct kset *kset_create_and_add(const char *name,
                                   const struct kset_uevent_ops *uevent_ops,
                                   struct kobject *parent_kobj);

```
```

  void kset_unregister(struct kset *k);

```
   来销毁它。这会从 sysfs 中移kset 并递减其引用计数。当引用计数归零时，kset 将被释放。由于可能仍存在kset 的其他引用，释放可能发生kset_unregister() 返回之后
使用 kset 的示例可参见内核树中`samples/kobject/kset-example.c` 文件
如果 kset 希望控制 kobject uevent 操作
```

  struct kset_uevent_ops {
          int (* const filter)(struct kobject *kobj);
          const char *(* const name)(struct kobject *kobj);
          int (* const uevent)(struct kobject *kobj, struct kobj_uevent_env *env);
  };


```
   filter 函数允许 kset 阻止为某个特kobject 向用户空间发uevent。如果函数返0，则不会发出 uevent
   name 函数会被调用，以覆盖 uevent 发送给用户空间kset 的默认名称。默认情况下名称kset 本身相同，但若提供了此函数，它可以覆盖该名称
   uevent 函数会在 uevent 即将发送给用户空间时被调用，以便向 uevent 添加更多环境变量
   有人可能会问，既然没有给出执行该功能的函数，kobject 究竟是如何被加入 kset 的。答案是这个任务kobject_add() 处理。当 kobject 被传kobject_add() 时，它的 kset 成员应指向该 kobject 将归属的 kset。kobject_add() 会处理其余事宜
   如果属于某个 kset kobject 没有设置kobject，它将被加入kset 的目录。并kset 的所有成员都一定位kset 目录中。如果在 kobject 被加入之前显式指定了kobject，那么该 kobject 会被注册kset，但会添加在kobject 之下

## kobject 的移

kobject 成功kobject 核心注册之后，当代码使用完毕时必须清理它。要做到这一点，调用 kobject_put()。这样，kobject 核心会自动清理该 kobject 分配的所有内存。如果已为该对象发送了 `KOBJ_ADD` uevent，则会发送相应的 `KOBJ_REMOVE` uevent，并且任何其sysfs 内务工作也会为调用者妥善处理
如果你需要对 kobject 进行两阶段删除（例如在你不能睡眠的时候需要销毁该对象），那么调用 kobject_del()，它会将 kobject sysfs 注销。这会让 kobject “不可见”，但它并未被清理，对象的引用计数仍然相同。稍后调kobject_put() 来完成与kobject 关联的内存清理
如果存在循环引用，kobject_del() 可用于丢弃对父对象的引用。在某些情况下父对象引用子对象是合法的。循环引用_必须_通过显式调用 kobject_del() 来打破，以便调用 release 函数，使前循环中的对象彼此释放

## 可参考的示例代码


有关正确使用 kset kobject 的更完整示例，请参阅示例程序 `samples/kobject/{kobject-example.c,kset-example.c}`，如果你选择`CONFIG_SAMPLE_KOBJECT`，它们将被构建为可加载模块