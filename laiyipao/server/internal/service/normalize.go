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
// # 边界
//
// 它是**响应边界**的归一，不是业务逻辑的改写：
// 「没有数据」和「数据是空的」在业务上是一回事，在 wire 上必须是同一回事。
func NormalizeNilSlices(v any) any {
	rv := reflect.ValueOf(v)
	if !rv.IsValid() {
		return v
	}
	if rv.Kind() == reflect.Pointer {
		if rv.IsNil() {
			return v
		}
		normalizeReflect(rv.Elem())
		return v
	}
	// ⚠️ 调用方全是**传值**（`c.JSON(NormalizeNilSlices(Resp{...}))`）。
	// 从 interface 取出的值不可寻址，字段 `CanSet()` 恒为 false ——
	// 直接 `normalizeReflect(rv)` 会让整个归一器空转，wire 仍是 `null`。
	// 复制成可写副本，归一后由返回值交还给调用方。
	c := reflect.New(rv.Type()).Elem()
	c.Set(rv)
	normalizeReflect(c)
	return c.Interface()
}

func normalizeReflect(v reflect.Value) {
	switch v.Kind() {
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
		// 玩家端响应的顶层形状是 `fiber.Map{"items": items}`。
		// 值不可寻址，必须复制成可写副本再回填。
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
