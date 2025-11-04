# Trying to compile core to `m68k-atari-mint-gcc`

Progress: Failing at instruction selection in LLVM. 

[Rustc Bug](https://github.com/rust-lang/rust/issues/143024)
[LLVM Bug](https://github.com/llvm/llvm-project/issues/146213)

Running the following:

```
cargo build -Z build-std=core -vv
```

Backtrace:

```
/home/takashi/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-67dd58abd922ea00.so(+0x3c22f0f) [0x7faf1a822f0f]
/lib64/libc.so.6(+0x1a070) [0x7faf16a28070]
/home/takashi/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libLLVM.so.21.1-rust-1.93.0-nightly(+0x763347d) [0x7faf1443347d]
/home/takashi/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libLLVM.so.21.1-rust-1.93.0-nightly(+0x762d5fc) [0x7faf1442d5fc]
/home/takashi/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libLLVM.so.21.1-rust-1.93.0-nightly(+0x7629bd2) [0x7faf14429bd2]
/home/takashi/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libLLVM.so.21.1-rust-1.93.0-nightly(+0x7e9d192) [0x7faf14c9d192]
/home/takashi/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libLLVM.so.21.1-rust-1.93.0-nightly(_ZN4llvm16SelectionDAGISel17CodeGenAndEmitDAGEv+0x811) [0x7faf14c9ba9d]
/home/takashi/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libLLVM.so.21.1-rust-1.93.0-nightly(_ZN4llvm16SelectionDAGISel20SelectAllBasicBlocksERKNS_8FunctionE+0x1782) [0x7faf14e5fcc2]
/home/takashi/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libLLVM.so.21.1-rust-1.93.0-nightly(_ZN4llvm16SelectionDAGISel20runOnMachineFunctionERNS_15MachineFunctionE+0x3b6) [0x7faf14f5ea76]
/home/takashi/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libLLVM.so.21.1-rust-1.93.0-nightly(+0x8b7a6c9) [0x7faf1597a6c9]
/home/takashi/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libLLVM.so.21.1-rust-1.93.0-nightly(_ZN4llvm13FPPassManager13runOnFunctionERNS_8FunctionE+0xa17) [0x7faf14c2e3d7]
/home/takashi/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libLLVM.so.21.1-rust-1.93.0-nightly(_ZN4llvm13FPPassManager11runOnModuleERNS_6ModuleE+0x28) [0x7faf14c2d8fe]
/home/takashi/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libLLVM.so.21.1-rust-1.93.0-nightly(_ZN4llvm6legacy15PassManagerImpl3runERNS_6ModuleE+0x2b1) [0x7faf151f2671]
/home/takashi/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-67dd58abd922ea00.so(+0x64b31c5) [0x7faf1d0b31c5]
/home/takashi/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-67dd58abd922ea00.so(+0x64b2dfb) [0x7faf1d0b2dfb]
/home/takashi/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-67dd58abd922ea00.so(+0x64b4ff5) [0x7faf1d0b4ff5]
/home/takashi/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-67dd58abd922ea00.so(_RINvNtNtCsaEKiIOnD17M_3std3sys9backtrace28___rust_begin_short_backtraceNCINvXs0_Csalb8s5xQiG1_18rustc_codegen_llvmNtB1g_18LlvmCodegenBackendNtNtNtCsdw7PpDJEA2A_17rustc_codegen_ssa6traits7backend19ExtraBackendMethods18spawn_named_threadNCINvNtNtB2k_4back5write19spawn_thin_lto_workB1O_E0uE0uEB1g_+0x175) [0x7faf1d11fdf5]
/home/takashi/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-67dd58abd922ea00.so(+0x6253539) [0x7faf1ce53539]
/home/takashi/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-67dd58abd922ea00.so(+0x6258def) [0x7faf1ce58def]
/lib64/libc.so.6(+0x71f54) [0x7faf16a7ff54]
/lib64/libc.so.6(+0xf532c) [0x7faf16b0332c]
```
