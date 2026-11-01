# ch2-lab：批处理系统与系统调用安全

依据官方教程 https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter2/5exercise.html 的实验练习，以及本仓库 ch2-lab 的 test1_write0/test1_write1 测例。使用分支既有的 2022-07-20 nightly，不将新版其他章节的扩展练习混入本实验。

实现 sys_write 的描述符和内存区间校验，非法请求返回 -1；checked_add 防止溢出。只有当前应用 ELF PT_LOAD 覆盖的连续映像范围和独立 4KiB 用户栈可以读取，应用范围包含 ELF 的 memsz（包括 BSS），不是只按二进制文件长度推断。验证完成后才创建切片，按字节输出，避免非 UTF-8 数据导致内核 panic。零长度请求不解引用地址。用户栈尺寸与分支测例的 4KiB 契约一致。

增加错误程序的异常指令地址及 stval 诊断。非法指令和越权存储只终止当前应用，后续应用继续运行；全部应用结束时正常关机。

普通函数调用在同一特权级用 jal/ret 传递控制，系统调用通过 ecall 进入受保护内核入口，保存现场并由 sret 返回。RISC-V 的 medeleg/mideleg 决定异常/中断委托；scause 表示原因，sepc 保存指令地址，stval 提供故障地址或指令信息。用户态执行 sret 或读取 sstatus 产生非法指令异常。独立内核栈避免用户伪造 sp 破坏保存现场过程。

Actions 独立矩阵运行基础批处理样例及原有 write 测例，检查非法应用隔离、数值计算、有效字符串输出、非法 FD、零地址与跨栈边界拒绝。保留完整 QEMU 日志和 ELF/bin 产物。本地不编译。
