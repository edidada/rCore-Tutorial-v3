"""Run the complete aggregate-branch usertests on real QEMU devices in CI."""
import re
import subprocess
from pathlib import Path
source = Path("user/src/bin/usertests.rs").read_text()
successes = source.split("static SUCC_TESTS:", 1)[1].split("static FAIL_TESTS:", 1)[0]
failures = source.split("static FAIL_TESTS:", 1)[1].split("use user_lib", 1)[0]
expected = (len(re.findall(r'\("', successes)), len(re.findall(r'\("', failures)))
command = ["qemu-system-riscv64", "-machine", "virt", "-bios", "default",
           "-serial", "stdio", "-display", "none", "-monitor", "none",
           "-global", "virtio-mmio.force-legacy=true",
           "-kernel", "os/target/riscv64gc-unknown-none-elf/release/os.bin",
           "-drive", "file=user/target/riscv64gc-unknown-none-elf/release/fs.img,if=none,format=raw,id=x0",
           "-device", "virtio-blk-device,drive=x0", "-device", "virtio-gpu-device",
           "-device", "virtio-keyboard-device", "-device", "virtio-mouse-device",
           "-device", "virtio-net-device,netdev=net0", "-netdev", "user,id=net0"]
Path("ci-logs").mkdir(exist_ok=True)
try:
    run = subprocess.run(command, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=900)
    output = run.stdout.decode(errors="replace")
except subprocess.TimeoutExpired as error:
    output = (error.stdout or b"").decode(errors="replace")
    Path("ci-logs/lab.log").write_text(output)
    print(output)
    raise SystemExit("QEMU usertests timed out")
Path("ci-logs/lab.log").write_text(output)
print(output)
assert run.returncode == 0, run.returncode
assert "Usertests passed!" in output and "Usertests failed!" not in output
assert "[kernel] Panicked" not in output
match = re.search(r"(\d+) of sueecssed apps, (\d+) of failed apps run correctly", output)
assert match and tuple(map(int, match.groups())) == expected, expected
assert "Idle process exit with exit_code 0" in output
print(f"AGGREGATE LAB PASSED: {expected[0]} success and {expected[1]} expected-failure cases")
