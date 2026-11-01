# ch5-lab：进程创建

实现 spawn 直接从目标 ELF 构造独立子进程，继承父子关系并进入就绪队列；不复制父地址空间后再 exec。失败查找返回 -1，路径逐页验证并限制长度。继承 mmap/munmap、跨页 get_time 和 sys_write 安全检查，处理读取页异常只终止当前进程。

Actions 通过真实用户 shell 分别运行 40 子进程 spawn 压力、wait/waitpid 退出码、mmap/unmap 和 sleep 测例，检查每个测试成功且 shell 返回码为零。使用 QEMU OpenSBI，空输入同时处理 SBI 的 -1 和 0。为并发 40 进程提供 QEMU 128MiB 可用物理内存。本地不编译。

fork 复制用户地址空间和上下文，子返回零、父返回 PID；spawn 直接加载另一程序。Zombie 保留退出码供父 wait 回收，孤儿交给 init；Arc/Weak 避免父子强引用环。
