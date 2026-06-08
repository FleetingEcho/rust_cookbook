// ============================================================
// Strategy Pattern — 将算法封装起来，使其可以互换
// 对比 Rust: 07_strategy.rs
// 运行: npx ts-node 07_strategy.ts
// ============================================================

interface Sorter {
  sort(data: number[]): number[];
  name: string;
}

class BubbleSort implements Sorter {
  name = "BubbleSort";
  sort(data: number[]): number[] {
    const arr = [...data];
    for (let i = 0; i < arr.length; i++)
      for (let j = 0; j < arr.length - i - 1; j++)
        if (arr[j] > arr[j + 1]) [arr[j], arr[j + 1]] = [arr[j + 1], arr[j]];
    return arr;
  }
}

class InsertionSort implements Sorter {
  name = "InsertionSort";
  sort(data: number[]): number[] {
    const arr = [...data];
    for (let i = 1; i < arr.length; i++) {
      const key = arr[i];
      let j = i - 1;
      while (j >= 0 && arr[j] > key) { arr[j + 1] = arr[j]; j--; }
      arr[j + 1] = key;
    }
    return arr;
  }
}

class SortContext {
  constructor(private strategy: Sorter) {}
  setStrategy(s: Sorter) { this.strategy = s; }
  run(data: number[]): number[] {
    console.log("策略:", this.strategy.name);
    return this.strategy.sort(data);
  }
}

// --- main ---
console.log("=== Strategy Pattern ===\n");

const raw = [5, 3, 8, 1, 9, 2, 7, 4, 6];

console.log("--- Class 方式 ---");
const ctx = new SortContext(new BubbleSort());
console.log("结果:", ctx.run(raw));

ctx.setStrategy(new InsertionSort());
console.log("结果:", ctx.run(raw), "\n");

// 函数式方式（最简洁）
console.log("--- 函数式（函数作策略）---");
const sortWith = (data: number[], fn: (a: number[]) => number[]) => fn(data);
console.log("升序:", sortWith(raw, a => [...a].sort((x, y) => x - y)));
console.log("降序:", sortWith(raw, a => [...a].sort((x, y) => y - x)));
