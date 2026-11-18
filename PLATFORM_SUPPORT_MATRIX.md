# Platform support for compiler 0.1.2

| Release target | Build | Packaged executable and installer | Native hosted workflow | Delivered |
|---|---|---|---|---|
| Linux x86-64 musl | PASS, two clean identical binaries | PASS on local Linux x86-64, shell installation and all examples | NOT EXECUTED | ZIP |
| Linux AArch64 musl | PASS, two clean identical cross-builds | PASS on qemu-aarch64 Linux userspace, host shell installation and all target CLI examples | NOT EXECUTED on physical ARM | ZIP |
| Windows x86-64 MSVC | NOT EXECUTED | NOT EXECUTED on Windows; PowerShell installer prepared | NOT EXECUTED | No binary |
| macOS x86-64 | NOT EXECUTED | NOT EXECUTED | NOT EXECUTED | No binary |
| macOS AArch64 | NOT EXECUTED | NOT EXECUTED | NOT EXECUTED | No binary |

The mandatory Cargo commands ran natively on x86_64-unknown-linux-gnu. The two distributable Linux executables target musl. Evidence covers the recorded environment and fixtures. The candidate workflow selects Windows Server 2022, Intel/ARM macOS 15 and Ubuntu 24.04 x64/ARM runners by label. It verifies packaged files, both build copies, version, help, scaffold, diagnostics, all examples and installer refusal paths before retaining a binary artifact. [Workflow](.github/workflows/distribution.yml) · [Runner reference](https://docs.github.com/en/actions/reference/runners/github-hosted-runners).
