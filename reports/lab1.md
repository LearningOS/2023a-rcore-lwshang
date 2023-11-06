# 实验报告

## 总结实现的功能

按照练习要求实现了系统调用 `sys_task_info()`。

为实现这个功能，需要让 `TaskManager` 可以在调度应用的过程中维护各个应用程序的相关信息。
因此，我在 `TaskControlBlock` 中添加了两个属性： 

```rust
pub struct TaskControlBlock {
    // 省略原有的两个属性
    pub dense_syscall_times: [u32; NUM_IMPLEMENTED_SYSCALLS], // [u32; 5]
    pub start_time: Option<usize>,
}
```

### 任务使用的系统调用及调用次数

`TaskInfo` 中 `syscall_times` 是一个长度为 500 的 `u32` 数组。但是我们只实现了五种系统调用，这个数组中只有五位有效，其他位置永远为零。

因此，在 `TaskControlBlock` 中，对应地定义了 `dense_syscall_times: [u32; NUM_IMPLEMENTED_SYSCALLS]` ，长度仅为 5。

并且在 `syscall` 模块中，我定义了 `syscall_id_to_dense()` 和 `syscall_id_from_dense()` 两个方法进行 `syscall id` 和在 dense 数组中的索引之间的转换。

### 系统调用时刻距离任务第一次被调度时刻的时长

在 `TaskControlBlock` 中，`start_time: Option<usize>` 记录了任务的开始时间。

在 `TaskManager` 实例化的时候，所有任务的 `start_time` 都初始化为 `None`。

在 `run_first_task()` 和 `run_next_task()` 初次调度到某个任务时，这个属性被赋值为 `Some(current_time)`。

当通过 `sys_task_info()` 获取当前任务运行时间，则通过当前时间和 `start_time` 做差得到。

## 简答作业

##### 1. 正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容 (运行 Rust 三个 bad 测例 (ch2b_bad_*.rs) ， 注意在编译时至少需要指定 LOG=ERROR 才能观察到内核的报错信息) ， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。

在 `ci-user` 目录下运行：
```
> make test CHAPTER=3 BASE=2 LOG=ERROR
...
[rustsbi] Implementation     : RustSBI-QEMU Version 0.2.0-alpha.2
...
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003c4, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
...
```

##### 2. 深入理解 trap.S 中两个函数 __alltraps 和 __restore 的作用，并回答如下问题:

###### 2.1. L40：刚进入 __restore 时，a0 代表了什么值。请指出 __restore 的两种使用情景.

刚进入 `__restore` 时，`a0` 代表了分配Trap上下文之后的内核栈栈顶。

`__restore` 主要是在 `trap_handler` 处理完成后从内核态返回用户态。当然，它也可用于在新任务开始时，转到用户态执行新任务。

###### 2.2. L43-L48：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。

```asm
ld t0, 32*8(sp)
ld t1, 33*8(sp)
ld t2, 2*8(sp)
csrw sstatus, t0
csrw sepc, t1
csrw sscratch, t2
```
* `sstatus`: `SPP` 等字段给出 Trap 发生之前 CPU 处在哪个特权级（S/U）等信息。返回用户态时，可以据此正确地设定回原来的特权级。
* `sepc`: 当 Trap 是一个异常的时候，记录 Trap 发生之前执行的最后一条指令的地址。进入用户态后，可以从该地址继续执行任务。
* `sscratch`:指向 Trap 时的用户栈栈顶。进入用户态需要通过该寄存器的值来恢复用户栈。

###### 2.3. L50-L56：为何跳过了 `x2` 和 `x4`

```asm
ld x1, 1*8(sp)
ld x3, 3*8(sp)
.set n, 5
.rept 27
   LOAD_GP %n
   .set n, n+1
.endr
```

因为在 `__alltraps` 保存 Trap 上下文通用寄存器的时候跳过了 `sp(x2)` 和 `tp(x4)`。对应地，恢复上下文的时候也要跳过它们。`sp(x2)` 已经通过 `csrw sscratch, t2` 恢复了。

###### 2.4. L60：该指令之后，`sp` 和 `sscratch` 中的值分别有什么意义？

```asm
csrrw sp, sscratch, sp
```

这是在 `__restore` 的结尾。

* `sp`: 用户栈栈顶。
* `sscratch`: 内核栈栈顶。

###### 2.5. `__restore`：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？

在最后的 `sret` 指令处发生状态切换。`sret` 的行为受 CSR 控制，而此时 `sstatus` 中的 `SPP` 字段已经被修改为了 `U`，所以会进入用户态。

###### 2.6. L13：该指令之后，`sp` 和 `sscratch` 中的值分别有什么意义？

```asm
csrrw sp, sscratch, sp
```

这是在 `__alltraps` 的开头。

* `sp`: 内核栈栈顶。
* `sscratch`: 用户栈栈顶。

###### 2.7. 从 U 态进入 S 态是哪一条指令发生的？

Trap 触发会导致用户态切换至内核态。

而 Trap 的原因有以下几种：

* 应用在用户态中主动进行系统调用，执行 `ecall` 指令
* 出现错误，例如 PageFault，IllegalInstruction
* 计时器触发


## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

rCore-Tutorial-Book 第三版

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
