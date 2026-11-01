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
if suite == "1":
    assert "Test write0 OK!" in output and "Test write1 OK!" in output
elif suite == "2":
    assert "Test set_priority OK!" in output
elif suite == "3":
    import re
    results = [(int(p), int(c)) for p, c in re.findall(r"priority = (\d+), exitcode = (\d+)", output)]
    assert len(results) == 6, results
    rates = [count / priority for priority, count in results]
    assert min(rates) > 0 and max(rates) / min(rates) < 2, results
elif suite == "4":
    assert "Test task_info OK!" in output
else:
    for power in [3, 5, 7]: assert f"Test power_{power} OK!" in output
print(f"CH3 SUITE {suite} PASSED")
