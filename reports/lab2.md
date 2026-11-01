# ch2-lab：批处理系统与系统调用安全

依据官方教程 https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter2/5exercise.html 的实验练习，以及本仓库 ch2-lab 的 test1_write0/test1_write1 测例。使用分支既有的 2022-07-20 nightly，不将新版其他章节的扩展练习混入本实验。

实现 sys_write 的描述符和内存区间校验，非法请求返回 -1；checked_add 防止溢出。只有当前应用 ELF PT_LOAD 覆盖的连续映像范围和独立 4KiB 用户栈可以读取，应用范围包含 ELF 的 memsz（包括 BSS），不是只按二进制文件长度推断。验证完成后才创建切片，按字节输出，避免非 UTF-8 数据导致内核 panic。零长度请求不解引用地址。用户栈尺寸与分支测例的 4KiB 契约一致。

增加错误程序的异常指令地址及 stval 诊断。非法指令和越权存储只终止当前应用，后续应用继续运行；全部应用结束时正常关机。

普通函数调用在同一特权级用 jal/ret 传递控制，系统调用通过 ecall 进入受保护内核入口，保存现场并由 sret 返回。RISC-V 的 medeleg/mideleg 决定异常/中断委托；scause 表示原因，sepc 保存指令地址，stval 提供故障地址或指令信息。用户态执行 sret 或读取 sstatus 产生非法指令异常。独立内核栈避免用户伪造 sp 破坏保存现场过程。

Actions 独立矩阵运行基础批处理样例及原有 write 测例，检查非法应用隔离、数值计算、有效字符串输出、非法 FD、零地址与跨栈边界拒绝。保留完整 QEMU 日志和 ELF/bin 产物。本地不编译。

## 实验问答

- `__restore` 入口 a0 是 TrapContext 地址。它既用于首次进入应用，也用于系统调用返回。恢复 sstatus 的 SPP/SPIE 决定返回用户态和中断使能，恢复 sepc 决定返回指令；先将用户 sp 装入 sscratch，再交换 sp/sscratch。
- x2 是 sp，不能在尚需读取内核栈上下文时提前恢复；x4 是 tp，本章没有使用线程指针，因此汇编未保存/恢复它。`csrrw sp, sscratch, sp` 在入口将用户 sp 保存到 sscratch、取出内核栈；恢复路径则相反。sret 才执行 S 到 U 的状态切换；ecall 或用户异常触发硬件 U 到 S 切换。
- trap 入口不能一般性地只保存调用约定中的被调用者保存寄存器：异步异常可能打断任意指令，调用者保存寄存器也包含活跃值。可针对已知同步调用快路径减少保存，或将浮点状态按需保存，但必须证明处理代码不会破坏省略的状态。
- 本 CI 使用 Ubuntu QEMU 附带的 OpenSBI，精确版本由每次 ci-logs 的固件启动横幅记录。两个特权 bad 程序分别执行 sret 和读取 sstatus，均应被 U 态非法指令检查拦截；零地址写触发存储故障，仅终止该应用。
