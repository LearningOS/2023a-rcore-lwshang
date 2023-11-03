# 实验报告

## 总结实现的功能

### 进程创建

结合 `fork` 和 `exec` 的实现，像 `fork` 一样创建新进程，但不需要复制父进程的地址空间，然后像 `exec` 一样加载程序并执行。

### stride 调度算法

重点是修改了优先队列 `ready_queue` 的定义，二元组的第一项是进程的 `stride`。`ready_queue` 按照 `stride` 由小到大排序。

```rust
pub struct TaskManager {
    ready_queue: VecDeque<(usize, Arc<TaskControlBlock>)>,
}
```

在 `TaskManager::add()` 中，先是按照算法更新该进程的 `stride`，然后插入 `ready_queue` 中的正确位置。

关于 `BigStride` 的选取，我选用了从 1 到 32 的所有整数的最小公倍数。这样可以尽可能使整数除法是准确的。


## 简答作业: stride 算法深入

##### 1. 实际情况是轮到 p1 执行吗？为什么？

p2 执行后，p2.stride 增加到 260，但 8 位无符号整数最大为 255，溢出变为 4。

4 < 255，仍然是 p2.stride 比较小，又轮到 p2 执行。

##### 2. 在不考虑溢出的情况下 , 在进程优先级全部 >= 2 的情况下，如果严格按照算法执行，那么 STRIDE_MAX – STRIDE_MIN <= BigStride / 2。为什么？

priority 越小，pass 越大，所以 pass 最大就是 PASS_MAX = BigStride / 2。

接下来应用数学归纳法来证明：

定义经过 n 轮调度时，stride 最大最小值的差为 
```
diff(n) = STRIDE_MAX(n) – STRIDE_MIN(n)
```

n = 0 时，一开始所有进程的 stride 都是 0，此时有
```
diff(0) = STRIDE_MAX(0) – STRIDE_MIN(0) = 0 - 0 = 0 <= PASS_MAX
```

假设 n = k 时命题成立，即 

```
diff(k) = STRIDE_MAX(k) – STRIDE_MIN(k) <= PASS_MAX
```

第 k 次调度时，有两种情况：

* 被调度的进程，其 stride 增加，成为集合中的最大值，此时有
```
STRIDE_MIN(k + 1) >= STRIDE_MIN(k)

STRIDE_MIN(k) + PASS_MAX >= STRIDE_MAX(k + 1)

diff(k + 1) = STRIDE_MAX(k + 1) – STRIDE_MIN(k + 1) <= PASS_MAX。
```

* 被调度后，stride 不是最大值，此时有

```
STRIDE_MIN(k + 1) >= STRIDE_MIN(k)

STRIDE_MIN(k + 1) = STRIDE_MAX(k)

diff(k + 1) = STRIDE_MAX(k + 1) – STRIDE_MIN(k + 1) <= STRIDE_MAX(k) – STRIDE_MIN(k) <= PASS_MAX
```

根据数学归纳法， `diff(n) <= PASS_MAX` 对任意正整数 n 均成立。

##### 3. 补全比较器

```rust
use core::cmp::Ordering;

struct Stride(u64);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0 == other.0 {
            Some(Ordering::Equal)
        } else if self.0 < other.0 {
            let diff = other.0 - self.0;
            if diff > u64::MAX / 2 {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Less)
            }
        } else {
            let diff = self.0 - other.0;
            if diff > u64::MAX / 2 {
                Some(Ordering::Less)
            } else {
                Some(Ordering::Greater)
            }
        }
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}
```

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

rCore-Tutorial-Book 第三版

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
