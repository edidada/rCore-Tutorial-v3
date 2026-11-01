import os, selectors, subprocess, time
from pathlib import Path
p = subprocess.Popen(["qemu-system-riscv64", "-machine", "virt", "-nographic", "-bios", "default", "-global", "virtio-mmio.force-legacy=true", "-drive", "file=user/target/riscv64gc-unknown-none-elf/release/fs.img,if=none,format=raw,id=x0", "-device", "virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0", "-kernel", "os/target/riscv64gc-unknown-none-elf/release/os.bin"], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
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
        ("test_file0", ["Test file0 OK!"]),
        ("test_file1", ["Test fstat OK!"]),
        ("test_file2", ["Test link OK!"]),
    ]:
        start = len(output)
        p.stdin.write((app + "\n").encode()); p.stdin.flush()
        for marker in markers: wait(marker, start)
        wait("exited with code 0", start)
        wait(">> ", start)
    print("CH7 EXPERIMENTS PASSED")
finally:
    p.kill(); p.wait()
    output += p.stdout.read().decode(errors="replace")
    Path("ci-logs").mkdir(exist_ok=True)
    Path("ci-logs/ch7.log").write_text(output)
    print(output)
