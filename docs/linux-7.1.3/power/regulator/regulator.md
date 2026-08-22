## 稳压器驱动接口（Regulator Driver Interface

稳压器驱动接口相对简单，旨在让稳压器驱动向核心框架注册其服务

## 注册


```

  struct regulator_dev *regulator_register(struct regulator_desc *regulator_desc,
					   const struct regulator_config *config);

```
这将向稳压器核心注册该稳压器的能力与操作
```

  void regulator_unregister(struct regulator_dev *rdev);


```
## 稳压器事

稳压器可以向
```

  int regulator_notifier_call_chain(struct regulator_dev *rdev,
				    unsigned long event, void *data);

```
