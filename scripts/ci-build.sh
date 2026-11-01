#!/usr/bin/env bash
set -euo pipefail
suite="${1:-0}"
make -C os kernel TEST="$suite" OBJCOPY=riscv64-unknown-elf-objcopy
riscv64-unknown-elf-objcopy --strip-all -O binary os/target/riscv64gc-unknown-none-elf/release/os os/target/riscv64gc-unknown-none-elf/release/os.bin
python3 scripts/ci-test.py "$suite"
