import os, selectors, subprocess, time
from pathlib import Path
p = subprocess.Popen(["qemu-system-riscv64", "-machine", "virt", "-nographic", "-bios", "default", "-kernel", "os/target/riscv64gc-unknown-none-elf/release/os.bin"], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
s = selectors.DefaultSelector(); s.register(p.stdout, selectors.EVENT_READ)
output = ""
def wait(text, start=0, timeout=120):
    global output
    end = time.monotonic() + timeout
    while text not in output[start:]:
        assert time.monotonic() < end, f"timeout: {text}"
        for key, _ in s.select(.2):
            data = os.read(key.fd, 65536)
            assert data, "QEMU exited early"
            output += data.decode(errors="replace")
    assert "[kernel] Panicked" not in output
try:
    wait(">> ")
    for app, markers in [
        ("test_spawn0", ["Test spawn0 OK!"]),
        ("test_spawn1", ["Test wait OK!", "Test waitpid OK!"]),
        ("test_mmap0", ["Test 04_1 OK!"]),
        ("test_mmap3", ["Test 04_4 test OK!"]),
        ("test_unmap", ["Test 04_5 ummap OK!"]),
        ("test_unmap2", ["Test 04_6 ummap2 OK!"]),
        ("test_sleep", ["Test sleep OK!"]),
        ("test_sleep1", ["Test sleep1 passed!"]),
        ("test_mail0", ["mail0 test OK!"]),
        ("test_mail1", ["mail1 test OK!"]),
        ("test_mail2", ["mail2 test OK!"]),
        ("test_mail3", ["mail3 test OK!"]),
    ]:
        start = len(output)
        p.stdin.write((app + "\n").encode()); p.stdin.flush()
        for marker in markers: wait(marker, start)
        wait("exited with code 0", start)
        wait(">> ", start)
    print("CH6 EXPERIMENTS PASSED")
finally:
    p.kill(); p.wait()
    output += p.stdout.read().decode(errors="replace")
    Path("ci-logs").mkdir(exist_ok=True)
    Path("ci-logs/ch6.log").write_text(output)
    print(output)
