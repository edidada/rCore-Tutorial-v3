# ch3-lab：任务信息与比例调度

官方实验：https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter3/5exercise.html 。官方实验要求 syscall 410；本仓库旧测例另包含 syscall 140 及 Stride，因此同时实现，保留原有测例。

## 相比上一实验的变化

1. 任务信息查询包含状态、系统调用计数和首次调度后的墙钟时长（毫秒）。
2. 每次进入 syscall 分派时计数，查询自身也计数，查询失败返回 -1。
3. 实现优先级及 Stride 选择，实际调度路径使用 u128 pass。
4. 继承应用映像与 4KiB 对齐栈边界检查，覆盖 write、get_time 和 task_info。
5. Actions 五组独立构建与 QEMU 测试。

## 实现与边界

TaskInfo/SyscallInfo 使用 repr(C)，TaskStatus 使用 repr(usize)。MAX_SYSCALL_NUM 为 500，计数存放静态内核区，查询时逐字段写入已验证用户缓冲区，避免在内核栈构建 8KiB 临时对象。运行时长从首次调度到退出/查询时的墙钟差值，包含被调度等待时间；未运行任务为零。

Stride 每次选 Ready 中 pass 最小者，推进 2^64/priority（至少为 1），权重下限 2，支持 isize::MAX，拒绝负数、0 和 1。QEMU 时钟频率按 virt 的 10MHz 设置，原 12.5MHz 假设会让时间测量偏差。

原有 build.py 忽略子构建错误，现改为失败立即终止。CI 测试原基础、write、set_priority、六个 Stride 应用，并新增 task_info 状态/计数/时长/非法地址测试。比例测试对 count/priority 的差异设宽容区间，避免把近似调度比例误当成每段窗口的严格保证。本地不编译。

## 问答

U 态执行 sret 或读取 sstatus 均触发非法指令异常；批处理实验的两个 bad 测例已独立验证。QEMU 启动 ROM 从复位向量跳到 0x80000000 固件，固件在 M 态配置内存保护、异常委托、mepc=0x80200000 和 mstatus.MPP=S，再通过 mret 进入内核。本 CI 使用系统 OpenSBI，版本与委托寄存器可见启动日志；没有把 RustSBI 的具体配置冒充本次固件运行结果。

协作式调度依赖任务主动 yield，抢占式调度依赖时钟中断。TrapContext 保存用户通用寄存器与特权返回状态，TaskContext 只保存内核上下文切换所需的 callee-saved 寄存器、ra、sp。切换前必须释放 UPSafeCell 的 RefMut，否则任务切回时会发生重复可变借用。中断进入时 SIE 自动关闭，本实验不启用内核中断嵌套。

调用栈打印为明确标注的选做题，本分支不实现该选做扩展。
