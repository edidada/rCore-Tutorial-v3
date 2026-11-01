#!/usr/bin/env bash
set -euo pipefail
make -C os fs-img TEST=1 OBJCOPY=riscv64-unknown-elf-objcopy
make -C os kernel OBJCOPY=riscv64-unknown-elf-objcopy
riscv64-unknown-elf-objcopy --strip-all -O binary os/target/riscv64gc-unknown-none-elf/release/os os/target/riscv64gc-unknown-none-elf/release/os.bin
python3 scripts/ci-test.py
