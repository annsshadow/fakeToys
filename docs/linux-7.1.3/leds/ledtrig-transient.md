## LED 瞬态触发器


leds 定时器触发器目前没有用于激活一次性定时器的接口。当前支持允许设置两定时器，一个用于指定保持开启状态的时长，另一个用于指定保持关闭状态的时长delay_on 值指LED 应保持在开启状态的时间段，其后delay_off 值指LED 保持在关闭状态的时长。开/关循环会一直重复，直到触发器被停用。目前没有提一次性激活机制来实现那些需要将开或关状态仅保持一次、然后永远停留在原始状的功能
如果没有一次性定时器接口，用户空间仍可使用定时器触发器来设置一个保持某状的定时器，但当用户空间应用程序崩溃或在不停用定时器的情况下退出时，硬件将
永久停留在该状态
瞬态触发器满足了一次性定时器激活的需求。瞬态触发器可以像其leds 触发器一被启用和禁用
当某led class 设备驱动注册自身时，它可以指定其支持的所leds 触发器以一个默认触发器。在注册期间，会调用默认触发器的激活例程。在注册 led class 设备
期间，LED 状态不会改变
当驱动注销时，会调用当前活动触发器的停用例程，并将 LED 状态更改为 LED_OFF
驱动挂起会将 LED 状态更改为 LED_OFF，而恢复不会改变该状态。请注意，挂起与
恢复操作和当前启用的触发器之间没有显式交互。在驱动处于挂起状态时，LED 状变更会被挂起。在驱动挂起时处于活动状态的任何定时器会继续运行，但无法实际改变
LED 状态。一旦驱动恢复，触发器会重新开始工作
LED 状态变更通过 brightness（这led class 设备的通用属性）来控制。当用户空间
通过 `echo 0 > brightness` brightness 设为 0 时，会导致当前触发器被停用
瞬态触发器使用标准的注册与注销接口。在触发器注册期间，对于将本触发器指定为
默认触发器的每个 led class 设备，都会调用触发器激活例程。在注册期间，LED 状不会改变，除非有另一个触发器处于活动状态，此时 LED 状态会变为 LED_OFF
在触发器注销期间，LED 状态会变为 LED_OFF
瞬态触发器的激活例程不会改LED 状态。它会创建其属性并进行初始化。瞬态触发器
的停用例程会先取消任何处于活动状态的定时器，再清理并移除其创建的属性。它会将
LED 状态恢复为非瞬态状态。当驱动挂起时，无论瞬态状态如何，LED 状态都会变LED_OFF
瞬态触发器可以从用户空间在 led class 上启用和禁用

```
	echo transient > trigger
	echo none > trigger

```
注意	新增一个属trigger state 以控制状态
该触发器导出三个属性：activate、state duration。当瞬态触发器被激活时，这属性会被设为默认值
- duration 允许以毫秒为单位设置定时器值。初始值为 0- activate 允许按需激活和停用duration 指定的定时器。初始值与默认值为 0  这使得可以在触发器激活后设置 duration- state 允许用户指定要为指定 duration 保持的瞬态状态
	activate
       - 一次性定时器激活机制		1 表示激活，0 表示停用		瞬态触发器启用时默认值为 0		以允许设duration
		activated 状态表示一个取值为指定
		duration 的定时器正在运行		deactivated 状态表示没有活动的定时		在运行
	duration
       - 一次性定时器值。设activate 时，duration 		用于启动一个只运行一次的定时器。该值不		被触发器更改，除非用户通过
		`echo new_value > duration` 进行设置
	state
       - 要被保持的瞬态状态。它有两个0 1 映射
		LED_OFF 映射LED_FULL。指定的状态在
		一次性定时器运行期间被保持，之后状态会变为
		瞬态状态的反状态（非瞬态状态）		如果 state = LED_FULL，当定时器超时时状态会
		回到 LED_OFF		如果 state = LED_OFF，当定时器超时时状态会
		回到 LED_FULL		请注意，在将状态更改为指定状态之前，不会检		当前 LED 状态		驱动可以依据其为 LED brightness_set() 接口
		（由 led brightness_set() 接口调用以控LED 状态）
		中定义的默认状态，将这些值映射为取反
当定时器超时时，activate 回到停用状态，duration 保留为所设置的值，供将来设activate 时再次使用。这将允许用户程序设置一次时间，并按需激活使其以指定值运一次。当定时器超时时，state 恢复为瞬态状态的反状态（非瞬态状态）
	=================   ===============================================
	echo 1 > activate   starts timer = duration when duration is not 0.
	echo 0 > activate   cancels currently running timer.
	echo n > duration   stores timer value to be used upon next
			    activate. Currently active timer if
			    any, continues to run for the specified time.
	echo 0 > duration   stores timer value to be used upon next
			    activate. Currently active timer if any,
			    continues to run for the specified time.
	echo 1 > state      stores desired transient state LED_FULL to be
			    held for the specified duration.
	echo 0 > state      stores desired transient state LED_OFF to be
			    held for the specified duration.
	=================   ===============================================

## 不支持的功能


- 定时器激活是一次性，且不支持延长或缩短定时器
## 示例


```
	echo transient > trigger
	echo n > duration
	echo 1 > state

```
```

	echo 1 > activate - start timer = duration to run once
	echo 1 > activate - start timer = duration to run once
	echo none > trigger

```
该触发器旨在用于以下示例用例
 - 用户空间应用程序LED 用作活动指示器 - 用户空间应用程序LED 用作一种看门狗指示器——只要应用程序存活，它就能让
   LED 保持点亮，若其崩溃，LED 将自动熄灭 - 任何需要瞬GPIO 输出的用户空间应用程序使用