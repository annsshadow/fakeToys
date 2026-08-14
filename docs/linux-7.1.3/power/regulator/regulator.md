## 绋冲帇鍣ㄩ┍鍔ㄦ帴鍙ｏ紙Regulator Driver Interface锛?

绋冲帇鍣ㄩ┍鍔ㄦ帴鍙ｇ浉瀵圭畝鍗曪紝鏃ㄥ湪璁╃ǔ鍘嬪櫒椹卞姩鍚戞牳蹇冩鏋舵敞鍐屽叾鏈嶅姟銆?

## 娉ㄥ唽


```

  struct regulator_dev *regulator_register(struct regulator_desc *regulator_desc,
					   const struct regulator_config *config);

```
杩欏皢鍚戠ǔ鍘嬪櫒鏍稿績娉ㄥ唽璇ョǔ鍘嬪櫒鐨勮兘鍔涗笌鎿嶄綔銆?
```

  void regulator_unregister(struct regulator_dev *rdev);


```
## 绋冲帇鍣ㄤ簨浠?

绋冲帇鍣ㄥ彲浠ュ悜
```

  int regulator_notifier_call_chain(struct regulator_dev *rdev,
				    unsigned long event, void *data);

```
