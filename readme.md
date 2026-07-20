```pwsh
cargo +nightly build -Z json-target-spec --target x86_64.json
rust-objcopy target/x86_64/debug/MatiOsShell -O elf64-x86-64 target/app.matiosexe  
```