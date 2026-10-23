// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import { test } from "node:test";
import { mapWithConcurrency } from "../src/runner.js";

test("结果顺序与输入一致（与完成先后无关）", async () => {
  const items = ["a", "b", "c", "d", "e"];
  // 完成顺序故意和输入顺序相反
  const delays: Record<string, number> = { a: 40, b: 30, c: 20, d: 10, e: 0 };
  const results = await mapWithConcurrency(items, 5, async (item) => {
    await new Promise((r) => setTimeout(r, delays[item]));
    return item.toUpperCase();
  });
  assert.deepEqual(results, ["A", "B", "C", "D", "E"]);
});

test("并发数不超过上限", async () => {
  const items = Array.from({ length: 8 }, (_, i) => i);
  let inFlight = 0;
  let maxInFlight = 0;
  await mapWithConcurrency(items, 3, async (item) => {
    inFlight++;
    maxInFlight = Math.max(maxInFlight, inFlight);
    await new Promise((r) => setTimeout(r, 5));
    inFlight--;
    return item;
  });
  assert.ok(maxInFlight > 1, `应当真的并发执行（实测峰值 ${maxInFlight}）`);
  assert.ok(maxInFlight <= 3, `并发数超限（实测峰值 ${maxInFlight}）`);
});

test("单个任务抛错不会阻止其余任务执行，错误最终向上传播", async () => {
  const items = [0, 1, 2, 3, 4];
  const attempted: number[] = [];
  await assert.rejects(
    () =>
      mapWithConcurrency(items, 2, async (item) => {
        attempted.push(item);
        await new Promise((r) => setTimeout(r, 5));
        if (item === 2) throw new Error(`boom-${item}`);
        return item;
      }),
    /boom-2/,
  );
  // 故障隔离：即使 2 号失败，其他任务仍全部跑完
  assert.deepEqual([...attempted].sort((a, b) => a - b), [0, 1, 2, 3, 4]);
});

test("全部任务成功时返回完整结果数组", async () => {
  const results = await mapWithConcurrency(["x", "y"], 2, async (item) => `${item}!`);
  assert.deepEqual(results, ["x!", "y!"]);
});

test("空输入直接返回空数组", async () => {
  const results = await mapWithConcurrency([], 4, async () => 1);
  assert.deepEqual(results, []);
});

test("限流为 1 时退化为串行", async () => {
  const order: number[] = [];
  let inFlight = 0;
  let maxInFlight = 0;
  await mapWithConcurrency([1, 2, 3], 1, async (item) => {
    inFlight++;
    maxInFlight = Math.max(maxInFlight, inFlight);
    order.push(item);
    await new Promise((r) => setTimeout(r, 5));
    inFlight--;
    return item;
  });
  assert.deepEqual(order, [1, 2, 3]);
  assert.equal(maxInFlight, 1);
});
