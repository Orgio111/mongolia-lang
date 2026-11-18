# Mongolia v0.1.2-ийг ажиллуулах

Нэгдсэн архитектурын v3.0 багцад ажилладаг хэлний хувилбар нь v0.1.2. [Нэгдсэн спецификаци](../UNIFIED_MASTER_SPECIFICATION.md) болон [Phase 10 шалгалтын тайлан](../BUILD_VERIFICATION_REPORT.md)-г үзнэ үү.

`mongolia-lang-0.1.2-source.zip` файлыг шинэ хавтаст задлаад терминалаар `mongolia-lang` хавтас руу орно. Rustup суусан байх шаардлагатай. Cargo нь энэ хавтасны `rust-toolchain.toml`-д заасан Rust хувилбарыг ашиглана. Эхний удаа Rust болон хамаарлуудыг татахад интернэт хэрэгтэй.

```sh
cd mongolia-lang
cargo build --workspace --locked
cargo run --locked -p mn-cli -- run examples/matmul_mn.mn
```

Хүлээх үр дүн нь `[[19.0, 22.0], [43.0, 50.0]]`. `examples/matmul.mn` доторх англи хувилбар ижил үр дүн өгнө.

`mn` командыг суулгаад шинэ програм үүсгэхэд дараах командуудыг ажиллуулна. Cargo-ийн bin хавтас PATH-д орсон байх шаардлагатай.

```sh
cargo install --path crates/mn-cli --locked
mn --version
mn init hello
mn run hello/main.mn
```

Програмыг ажиллуулахаас өмнө төрлүүд болон тензорын хэмжээсийг шалгахын тулд `mn check` хэрэглэнэ. Дараах жишээний дотоод хэмжээсүүд 3 ба 2 тул шалгалт алдааны код 1 буцаана.

```sh
mn check examples/negative/shape_mismatch_mn.mn --lang mn
```

Бүх тест болон жишээг шалгах команд нь `python3 scripts/verify.py`. Хэлний дүрэм SPEC.md файлд, тестийн шаардлагууд docs/PHASE_2_ACCEPTANCE.md файлд бий.

Linux дээр задалсан `mongolia-lang` хавтаст `sha256sum -c SHA256SUMS` ажиллуулж
файл бүрийн бүрэн бүтэн байдлыг шалгана. Түр хавтаст суулгаж жишээ бүрийг
ажиллуулах шалгалтын команд нь `python3 scripts/verify_install.py`.
