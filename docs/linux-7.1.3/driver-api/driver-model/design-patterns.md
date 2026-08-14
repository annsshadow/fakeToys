## 设备驱动设计模式


本文档描述了设备驱动中一些常见的设计模式。子系统维护者很可能会要求驱动开发者遵循这些设计模式。

1. 状态容器（State Container）
2. container_of()

#### 1. 状态容器


虽然内核中有少数设备驱动假定它们在某个系统上只会被 probe() 一次（单例），但习惯上应假定驱动所绑定的设备会出现多个实例。这意味着 probe() 函数与所有回调都必须是可重入的。

实现这一点最常见的方法是使用状态容器设计
```

  struct foo {
      spinlock_t lock; /* 示例成员 */
      (...)
  };

  static int foo_probe(...)
  {
      struct foo *foo;

      foo = devm_kzalloc(dev, sizeof(*foo), GFP_KERNEL);
      if (!foo)
          return -ENOMEM;
      spin_lock_init(&foo->lock);
      (...)
  }

```

每次调用 probe() 时，这会在内存中创建一个 struct foo 的实例。这就是该设备驱动实例的状态容器。当然，之后有必要始终将这个状态实例传递给所有需要访问该状态及其成员的函数。

例如，如果驱动正在注册一个中断处理函数，你会
```

  static irqreturn_t foo_handler(int irq, void *arg)
  {
      struct foo *foo = arg;
      (...)
  }

  static int foo_probe(...)
  {
      struct foo *foo;

      (...)
      ret = request_irq(irq, foo_handler, 0, "foo", foo);
  }

```

这样，在中断处理函数中你总能取回指向正确 foo 实例的指针。

#### 2. container_of()


```

  struct foo {
      spinlock_t lock;
      struct workqueue_struct *wq;
      struct work_struct offload;
      (...)
  };

  static void foo_work(struct work_struct *work)
  {
      struct foo *foo = container_of(work, struct foo, offload);

      (...)
  }

  static irqreturn_t foo_handler(int irq, void *arg)
  {
      struct foo *foo = arg;

      queue_work(foo->wq, &foo->offload);
      (...)
  }

  static int foo_probe(...)
  {
      struct foo *foo;

      foo->wq = create_singlethread_workqueue("foo-wq");
      INIT_WORK(&foo->offload, foo_work);
      (...)
  }

```

对于 hrtimer 或类似的、在回调中返回单个参数（指向结构体成员的指针）的情况，设计模式是相同的。

container_of() 是 <linux/container_of.h> 中定义的宏。

container_of() 所做的事情是，利用标准 C 的 offsetof() 宏通过简单的减法，从一个指向成员的指针获得指向包含它的结构体的指针，从而实现类似于面向对象的行为。注意，被包含的成员不能是（指向另一结构的）指针，而必须是实际的成员才能工作。

我们可以看到，通过这种方式我们避免了持有指向 struct foo * 实例的全局指针，同时将传递给 work 函数的参数数量保持在单个指针。
