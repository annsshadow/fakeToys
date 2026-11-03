// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

import "reflect"

// NormalizeNilSlices 把响应结构体里的 **nil 切片 / nil map** 换成空集合。
//
// # 为什么需要它
//
// Go 的 `encoding/json` 把 nil 切片编码成 `null`、nil map 也编码成 `null`，
// 而空切片编码成 `[]`、空 map 编码成 `{}`。
//
// 于是「这个列表为空」在 wire 上有**两种**表示：
//
//	[]   ← 服务端返回了空切片
//	null ← 服务端返回了 nil 切片
//
// 而生成的 TypeScript 类型只有一个：`T[]`（**不是** `T[] | null`）——
// 因为类型是从 Go 的 struct 派生的，而 struct 声明的是切片。
//
// 后果：前端拿到 `null` 后按数组用 → `null.length` → **TypeError** →
// 整个抽屉/页面渲染中断。
//
// # 为什么在**服务端**归一，而不是让前端到处判空
//
//  1. 前端要判空的地方有十几处，漏一处就是一个崩溃点；
//  2. 更根本的：类型是从服务端 struct 生成的，服务端就该**让 wire 符合
//     自己的类型声明** —— 否则「唯一事实源」又是假的。
//
// # 第 64 轮：必须先深拷贝，不能就地改写
//
// 原实现只把结构体**浅拷贝**一层，然后递归 `SetMapIndex` / 改切片元素 ——
// map 与切片底层数组仍与入参共享。命中缓存的响应（如 `/admin/economy`
// 缓存 30 秒内的 `AdminEconomyResp`）在多个并发请求间是**同一个对象**：
// 一个 goroutine 正在 `MapRange` 遍历 `price`，另一个在 `SetMapIndex` 写它，
// 触发 `fatal error: concurrent map iteration and map write`，**整个进程退出**
// （这不是 panic，中间件 recover 不了）。
//
// 实测：对 `/admin/economy` 打 10 并发，8 秒内进程必挂。
// 所以这里先 `deepCopy` 出**完全私有**的副本再归一，入参永远只读。
//
// # 边界
//
// 它是**响应边界**的归一，不是业务逻辑的改写：
// 「没有数据」和「数据是空的」在业务上是一回事，在 wire 上必须是同一回事。
func NormalizeNilSlices(v any) any {
	rv := reflect.ValueOf(v)
	if !rv.IsValid() {
		return v
	}
	c := deepCopy(rv)
	normalizeReflect(c)
	return c.Interface()
}

// deepCopy 递归复制 struct / slice / map / pointer / interface。
//
// 目的只有一个：让归一化只写**自己的副本**，绝不碰调用方（尤其是缓存）里的
// map 与切片底层数组。未导出字段不递归（归一器也改不了它们，CanSet=false），
// 由 `out.Set(v)` 原样带过来。
func deepCopy(v reflect.Value) reflect.Value {
	switch v.Kind() {
	case reflect.Pointer:
		if v.IsNil() {
			return v
		}
		out := reflect.New(v.Type().Elem())
		out.Elem().Set(deepCopy(v.Elem()))
		return out
	case reflect.Interface:
		if v.IsNil() {
			return v
		}
		out := reflect.New(v.Type()).Elem()
		out.Set(deepCopy(v.Elem()))
		return out
	case reflect.Struct:
		out := reflect.New(v.Type()).Elem()
		out.Set(v)
		for i := 0; i < v.NumField(); i++ {
			if v.Type().Field(i).PkgPath != "" {
				continue
			}
			out.Field(i).Set(deepCopy(v.Field(i)))
		}
		return out
	case reflect.Slice:
		if v.IsNil() {
			return v
		}
		out := reflect.MakeSlice(v.Type(), v.Len(), v.Len())
		for i := 0; i < v.Len(); i++ {
			out.Index(i).Set(deepCopy(v.Index(i)))
		}
		return out
	case reflect.Map:
		if v.IsNil() {
			return v
		}
		out := reflect.MakeMapWithSize(v.Type(), v.Len())
		iter := v.MapRange()
		for iter.Next() {
			out.SetMapIndex(deepCopy(iter.Key()), deepCopy(iter.Value()))
		}
		return out
	default:
		return v
	}
}

func normalizeReflect(v reflect.Value) {
	switch v.Kind() {
	case reflect.Pointer:
		// 指针入参（旧用法）在深拷贝后同样要走到被指向的结构体。
		if !v.IsNil() {
			normalizeReflect(v.Elem())
		}
	case reflect.Struct:
		for i := 0; i < v.NumField(); i++ {
			f := v.Field(i)
			if !f.CanSet() {
				continue
			}
			normalizeField(f)
		}
	case reflect.Slice:
		// 切片**元素**里的 nil 切片也要归一（`[]AdminBattleView` 每行自己还有切片）
		if v.IsNil() {
			return
		}
		for i := 0; i < v.Len(); i++ {
			normalizeReflect(v.Index(i))
		}
	case reflect.Map:
		// 这里是**副本**内部的 map（deepCopy 保证），就地写是安全的。
		// 玩家端响应的顶层形状是 `fiber.Map{"items": items}`：
		// map 值不可寻址，必须复制成可写副本再回填。
		if v.IsNil() {
			return
		}
		iter := v.MapRange()
		for iter.Next() {
			val := iter.Value()
			cp := reflect.New(val.Type()).Elem()
			cp.Set(val)
			normalizeField(cp)
			v.SetMapIndex(iter.Key(), cp)
		}
	}
}

func normalizeField(f reflect.Value) {
	switch f.Kind() {
	case reflect.Interface:
		// `map[string]any` / `any` 字段里实际装的是具体类型；拆包后归一再装回。
		if f.IsNil() {
			return
		}
		inner := f.Elem()
		cp := reflect.New(inner.Type()).Elem()
		cp.Set(inner)
		normalizeField(cp)
		f.Set(cp)
	case reflect.Slice:
		if f.IsNil() {
			f.Set(reflect.MakeSlice(f.Type(), 0, 0))
		}
		normalizeReflect(f)
	case reflect.Map:
		if f.IsNil() {
			f.Set(reflect.MakeMap(f.Type()))
			return
		}
		normalizeReflect(f)
	case reflect.Struct:
		normalizeReflect(f)
	case reflect.Pointer:
		if !f.IsNil() {
			normalizeReflect(f.Elem())
		}
	}
}
