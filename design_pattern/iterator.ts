// ============================================================
// Iterator Pattern — 统一遍历接口，用 Generator 实现自定义迭代器
// 对比 Rust: 10_iterator.rs
// 运行: npx ts-node 10_iterator.ts
// ============================================================

// 自定义步进范围（Generator）
function* stepRange(start: number, end: number, step: number): Generator<number> {
  for (let i = start; i < end; i += step) yield i;
}

// 斐波那契（无限 Generator）
function* fibonacci(): Generator<number> {
  let [a, b] = [0, 1];
  while (true) { yield b; [a, b] = [b, a + b]; }
}

function take<T>(gen: Generator<T>, n: number): T[] {
  const result: T[] = [];
  for (const val of gen) {
    result.push(val);
    if (result.length >= n) break;
  }
  return result;
}

// --- main ---
console.log("=== Iterator Pattern ===\n");

console.log("--- 自定义步进范围 ---");
console.log("0..20 step 3:", [...stepRange(0, 20, 3)]);

console.log("\n--- 链式操作 ---");
// 注意：TS 每步都产生新数组（非惰性）
const result = [...stepRange(0, 20, 1)]
  .filter(n => n % 2 === 0)
  .map(n => n * n)
  .slice(0, 5);
console.log("前5个偶数的平方:", result);

console.log("\n--- 斐波那契（无限 + take 截断）---");
console.log("前10项:", take(fibonacci(), 10));

console.log("\n--- 数组内置方法 ---");
const nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
console.log("sum:    ", nums.reduce((a, b) => a + b, 0));
console.log("product:", nums.reduce((a, b) => a * b, 1));
console.log("evens:  ", nums.filter(x => x % 2 === 0));
console.log("any > 5:", nums.some(x => x > 5));
console.log("all > 0:", nums.every(x => x > 0));
console.log("max:    ", Math.max(...nums));

console.log("\n--- zip（手动实现）---");
const names  = ["Alice", "Bob", "Carol"];
const scores = [95, 87, 92];
console.log(names.map((name, i) => [name, scores[i]]));

console.log("\n--- flatMap ---");
const sentences = ["hello world", "foo bar baz"];
console.log(sentences.flatMap(s => s.split(" ")));

// Rust 关键差异：
// - Rust Iterator 惰性，链式操作不产生中间集合，性能更好
// - TS .filter().map() 每步新建数组
