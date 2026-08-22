## The Linux Kernel API


## Basic C Library Functions


编写驱动时，一般不能使用来C 库的例程。其中一些函数被发现具有普遍的实用性，下面列出它们。这些函数的行为可能ANSI 定义的略有不同，这些差异在文中已注明
### String Conversions


   :export:

   :functions: kstrtol kstrtoul

   :export:

   :export:

### String Manipulation


   :internal:

   :export:

   :internal:

   :functions: kstrdup kstrdup_const kstrndup kmemdup kmemdup_nul memdup_user
               vmemdup_user strndup_user memdup_user_nul

## Basic Kernel Library Functions


Linux 内核提供了更多基础的实用函数
### Bit Operations


   :internal:

   :internal:

   :internal:

### Bitmap Operations


   :doc: bitmap introduction

   :doc: declare bitmap

   :doc: bitmap overview

   :doc: bitmap bitops

   :export:

   :internal:

   :internal:

### Command-line Parsing


   :export:

### Error Pointers


   :internal:

### Sorting


   :export:

   :export:

### Text Searching


   :doc: ts_intro

   :export:

   :functions: textsearch_find textsearch_next \
               textsearch_get_pattern textsearch_get_pattern_len

## CRC and Math Functions in Linux


### Arithmetic Overflow Checking


   :internal:

### CRC Functions


   :export:

   :export:

   :export:

   :export:

   :export:

   :export:

   :export:


### Base 2 log and power Functions


   :internal:

### Integer log and power Functions



   :export:

   :export:

### Division Functions


   :functions: do_div

   :internal:

   :export:

### UUID/GUID


   :export:

## Kernel IPC facilities


### IPC utilities


   :internal:

## FIFO Buffer


### kfifo interface


   :internal:

## relay interface support


Relay 接口支持旨在为工具和设施提供一种高效机制，将大量数据从内核空间传送到用户空间
### relay interface


   :export:

   :internal:

## Module Support


### Kernel module auto-loading


   :export:

### Module debugging


   :doc: module debugging statistics overview

######## dup_failed_modules - tracks duplicate failed modules


   :doc: dup_failed_modules - tracks duplicate failed modules

######## module statistics debugfs counters


   :doc: module statistics debugfs counters

### Inter Module support


更多信息请参kernel/module/ 下的文件
## Hardware Interfaces


### DMA Channels


   :export:

### Resources Management


   :internal:

   :export:

### MTRR Handling


   :export:

## Security Framework


   :internal:

   :export:

## Audit Interfaces


   :export:

   :internal:

   :internal:

## Accounting Framework


   :internal:

## Block Devices


   :export:

   :internal:

   :export:

   :internal:

   :export:

   :export:

   :export:

   :export:

   :internal:

   :internal:

   :export:

   :export:

## Char devices


   :export:

## Clock Framework


时钟框架（clock framework）定义了编程接口，以支持对系统时钟树（clock tree）的软件管理。该框架广泛用于片上系统（SOC）平台，以支持电源管理以及各种可能需要自定义时钟频率的设备。请注意，这时钟"与时间保持或实时时钟（RTC）无关，后两者各有独立的框架。这`struct clk <clk>` 实例可用于管理例如一96 MHz 的信号，该信号用于将数据位移入和移出外设或总线，或以其他方式触发系统硬件中的同步状态机转换
电源管理通过显式的软件时钟门控（software clock gating）来支持：未使用的时钟被禁用，这样系统就不会浪费功耗去改变未被主动使用的晶体管状态。在某些系统上，这可能由硬件时钟门控作为支撑，即时钟在软件中未被禁用的情况下被门控。已上电但未提供时钟的芯片部分可能能够保留其最后状态。这种低功耗状态通常称为**保持模式（retention mode*。该模式仍然会产生泄漏电流，尤其是在更精细的电路几何尺寸下，但对CMOS 电路而言，功耗主要由时钟驱动的状态变化消耗
注重功耗的驱动仅在其管理的设备处于活动使用状态时启用其时钟。此外，系统睡眠状态通常根据哪些时钟域处于活动状态而不同：虽然"standby"状态可能允许来自多个活动域的唤醒，mem"（挂起到 RAM）状态可能需要更全面地关闭源自高PLL 和振荡器的时钟，从而限制可能的唤醒事件源数量。驱动的 suspend 方法可能需要了解目标睡眠状态上与系统相关的时钟约束
某些平台支持可编程时钟发生器。这些可被各种外部芯片使用，例如其他 CPU、多媒体编解码器，以及对接口时钟有严格要求的设备
   :internal:

## Synchronization Primitives


### Read-Copy Update (RCU)




















