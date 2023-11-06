# 实验报告

## 总结实现的功能

### 死锁检测

我将银行家算法中需要用到的数据抽象为：

```rust
/// Resource to be shared amoung Threads of a Process
pub trait Resource {
    /// The available amount of this resource.
    fn get_available(&self) -> usize;
    /// The amounts allocated to Tids
    fn get_allocation(&self) -> Vec<(Tid, usize)>;
    /// The amount needed by Tids
    fn get_need(&self) -> Vec<(Tid, usize)>;
}
```

并对 `Mutex` 和 `Semaphore` 都实现了这个 trait。

最后在 `ProcessControlBlockInner` 中增加 `check_resource(res_id)` 方法。 其中会遍历本进程的所有 `Mutex` 和 `Semaphore`。获取 `Resource` trait 对应的数据，构造算法中用到的向量和矩阵。

### enable_deadlock_detect

只需要在 `ProcessControlBlockInner` 中增加一个对应的 bool 成员。


## 问答作业:

##### 1. 在我们的多线程实现中，当主线程 (即 0 号线程) 退出时，视为整个进程退出， 此时需要结束该进程管理的所有线程并回收其资源。 

###### 需要回收的资源有哪些？ 

每个线程的 `TaskUserRes`，`TaskContext`.

整个进程的 `MemorySet`, `fd_table` 中的文件， `Mutex` 和 `Semaphore` 等用于线程并发的数据结构。

###### 其他线程的 TaskControlBlock 可能在哪些位置被引用，分别是否需要回收，为什么？

`MutexBlocking` 和 `Semaphore` 的等待队列中可能有其他线程 TCB 的引用。在进程退出的过程中，这些数据结构都会释放，进而 TCB 的引用计数归零，TCB 最终也会释放。

##### 2. 对比以下两种 Mutex.unlock 的实现，二者有什么区别？这些区别可能会导致什么问题？

两种实现主要区别在于释放锁和唤醒其他线程的先后顺序不同。

`Mutex1` 先解锁，后唤醒其他线程。若在这两步之间发生中断，从中断恢复后，原线程和新唤醒线程可能发生 Race Condition。

`Mutex2` 先唤醒其他线程，最后解锁。这样是正确的实现，不会产生上述问题。

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

rCore-Tutorial-Book 第三版

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
