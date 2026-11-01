import subprocess
import sys
from pathlib import Path
suite = sys.argv[1]
command = ["qemu-system-riscv64", "-machine", "virt", "-nographic", "-bios", "default",
           "-kernel", "os/target/riscv64gc-unknown-none-elf/release/os.bin"]
Path("ci-logs").mkdir(exist_ok=True)
try:
    run = subprocess.run(command, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=90)
    output = run.stdout.decode(errors="replace")
except subprocess.TimeoutExpired as error:
    output = (error.stdout or b"").decode(errors="replace")
    Path(f"ci-logs/suite-{suite}.log").write_text(output)
    print(output)
    raise SystemExit("QEMU timeout")
Path(f"ci-logs/suite-{suite}.log").write_text(output)
print(output)
assert run.returncode == 0
assert "All applications completed!" in output
assert "Panicked" not in output and "FAIL:" not in output
if suite == "2":
    for marker in ["Test 04_1 OK!", "Test 04_4 test OK!", "Test 04_6 ummap2 OK!"]: assert marker in output
    assert "fail!" not in output
else:
    assert "OK!" in output
print(f"CH4 SUITE {suite} PASSED")
