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
	if rv.Kind() == reflect.Pointer {
		if rv.IsNil() {
			return v
		}
		rv = rv.Elem()
	}
	normalizeReflect(rv)
	return v
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
	}
}

func normalizeField(f reflect.Value) {
	switch f.Kind() {
	case reflect.Slice:
		if f.IsNil() {
			f.Set(reflect.MakeSlice(f.Type(), 0, 0))
		}
		normalizeReflect(f)
	case reflect.Map:
		if f.IsNil() {
			f.Set(reflect.MakeMap(f.Type()))
		}
	case reflect.Struct:
		normalizeReflect(f)
	case reflect.Pointer:
		if !f.IsNil() {
			normalizeReflect(f.Elem())
		}
	}
}
