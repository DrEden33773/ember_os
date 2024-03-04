# Diff Details

Date : 2024-03-04 23:29:16

Directory /home/eden/ProjectRepo/my_ros

Total : 76 files,  1479 codes, 187 comments, 246 blanks, all 1912 lines

[Summary](results.md) / [Details](details.md) / [Diff Summary](diff.md) / Diff Details

## Files
| filename | language | code | comment | blank | total |
| :--- | :--- | ---: | ---: | ---: | ---: |
| [.cargo/config.toml](/.cargo/config.toml) | TOML | 13 | 0 | 6 | 19 |
| [Cargo.lock](/Cargo.lock) | TOML | 322 | 2 | 45 | 369 |
| [Cargo.toml](/Cargo.toml) | TOML | 61 | 7 | 10 | 78 |
| [README.md](/README.md) | source.markdown.math | 11 | 0 | 8 | 19 |
| [TODO.md](/TODO.md) | source.markdown.math | 3 | 0 | 3 | 6 |
| [benchmarks/TODO.md](/benchmarks/TODO.md) | source.markdown.math | 3 | 0 | 2 | 5 |
| [rustfmt.toml](/rustfmt.toml) | TOML | 1 | 0 | 1 | 2 |
| [src/allocator.rs](/src/allocator.rs) | Rust | 94 | 11 | 28 | 133 |
| [src/allocator/bump.rs](/src/allocator/bump.rs) | Rust | 50 | 18 | 11 | 79 |
| [src/allocator/fixed_size_block.rs](/src/allocator/fixed_size_block.rs) | Rust | 72 | 20 | 13 | 105 |
| [src/allocator/linked_list.rs](/src/allocator/linked_list.rs) | Rust | 104 | 43 | 20 | 167 |
| [src/demo/concurrency.rs](/src/demo/concurrency.rs) | Rust | 86 | 2 | 11 | 99 |
| [src/demo/cpu_exceptions.rs](/src/demo/cpu_exceptions.rs) | Rust | 4 | 0 | 1 | 5 |
| [src/demo/double_fault.rs](/src/demo/double_fault.rs) | Rust | 6 | 0 | 1 | 7 |
| [src/demo/heap_allocation.rs](/src/demo/heap_allocation.rs) | Rust | 22 | 0 | 8 | 30 |
| [src/demo/memory.rs](/src/demo/memory.rs) | Rust | 36 | 6 | 6 | 48 |
| [src/demo/mod.rs](/src/demo/mod.rs) | Rust | 13 | 3 | 4 | 20 |
| [src/demo/println_eprintln.rs](/src/demo/println_eprintln.rs) | Rust | 8 | 0 | 2 | 10 |
| [src/exit.rs](/src/exit.rs) | Rust | 13 | 0 | 2 | 15 |
| [src/gdt.rs](/src/gdt.rs) | Rust | 46 | 0 | 8 | 54 |
| [src/interrupts.rs](/src/interrupts.rs) | Rust | 121 | 26 | 24 | 171 |
| [src/lib.rs](/src/lib.rs) | Rust | 74 | 6 | 11 | 91 |
| [src/main.rs](/src/main.rs) | Rust | 41 | 2 | 12 | 55 |
| [src/memory.rs](/src/memory.rs) | Rust | 94 | 31 | 22 | 147 |
| [src/prelude.rs](/src/prelude.rs) | Rust | 0 | 0 | 2 | 2 |
| [src/serial.rs](/src/serial.rs) | Rust | 32 | 3 | 6 | 41 |
| [src/task/executor.rs](/src/task/executor.rs) | Rust | 103 | 5 | 17 | 125 |
| [src/task/keyboard.rs](/src/task/keyboard.rs) | Rust | 93 | 8 | 12 | 113 |
| [src/task/mod.rs](/src/task/mod.rs) | Rust | 68 | 0 | 13 | 81 |
| [src/task/simple_executor.rs](/src/task/simple_executor.rs) | Rust | 44 | 0 | 8 | 52 |
| [src/test_framework.rs](/src/test_framework.rs) | Rust | 11 | 0 | 3 | 14 |
| [src/utils/algorithms/graph_interface/cached_greedy_extreme_path.rs](/src/utils/algorithms/graph_interface/cached_greedy_extreme_path.rs) | Rust | 441 | 75 | 64 | 580 |
| [src/utils/algorithms/graph_interface/greedy_extreme_path.rs](/src/utils/algorithms/graph_interface/greedy_extreme_path.rs) | Rust | 409 | 77 | 64 | 550 |
| [src/utils/algorithms/graph_interface/mod.rs](/src/utils/algorithms/graph_interface/mod.rs) | Rust | 96 | 0 | 15 | 111 |
| [src/utils/algorithms/mod.rs](/src/utils/algorithms/mod.rs) | Rust | 1 | 0 | 1 | 2 |
| [src/utils/collections/lru_cache.rs](/src/utils/collections/lru_cache.rs) | Rust | 212 | 26 | 35 | 273 |
| [src/utils/collections/mod.rs](/src/utils/collections/mod.rs) | Rust | 3 | 0 | 2 | 5 |
| [src/utils/collections/trie.rs](/src/utils/collections/trie.rs) | Rust | 81 | 0 | 15 | 96 |
| [src/utils/mod.rs](/src/utils/mod.rs) | Rust | 2 | 0 | 1 | 3 |
| [src/vga_buffer.rs](/src/vga_buffer.rs) | Rust | 292 | 19 | 44 | 355 |
| [tests/heap_allocation.rs](/tests/heap_allocation.rs) | Rust | 62 | 1 | 11 | 74 |
| [tests/should_panic.rs](/tests/should_panic.rs) | Rust | 29 | 0 | 6 | 35 |
| [tests/stack_overflow.rs](/tests/stack_overflow.rs) | Rust | 52 | 3 | 12 | 67 |
| [/home/eden/ProjectsRepo/my_ros/.cargo/config.toml](//home/eden/ProjectsRepo/my_ros/.cargo/config.toml) | TOML | -7 | 0 | -3 | -10 |
| [/home/eden/ProjectsRepo/my_ros/Cargo.lock](//home/eden/ProjectsRepo/my_ros/Cargo.lock) | TOML | -253 | -2 | -36 | -291 |
| [/home/eden/ProjectsRepo/my_ros/Cargo.toml](//home/eden/ProjectsRepo/my_ros/Cargo.toml) | TOML | -45 | 0 | -9 | -54 |
| [/home/eden/ProjectsRepo/my_ros/README.md](//home/eden/ProjectsRepo/my_ros/README.md) | source.markdown.math | -6 | 0 | -5 | -11 |
| [/home/eden/ProjectsRepo/my_ros/rustfmt.toml](//home/eden/ProjectsRepo/my_ros/rustfmt.toml) | TOML | -1 | 0 | -1 | -2 |
| [/home/eden/ProjectsRepo/my_ros/src/allocator.rs](//home/eden/ProjectsRepo/my_ros/src/allocator.rs) | Rust | -90 | -12 | -19 | -121 |
| [/home/eden/ProjectsRepo/my_ros/src/allocator/bump.rs](//home/eden/ProjectsRepo/my_ros/src/allocator/bump.rs) | Rust | -48 | -21 | -9 | -78 |
| [/home/eden/ProjectsRepo/my_ros/src/allocator/fixed_size_block.rs](//home/eden/ProjectsRepo/my_ros/src/allocator/fixed_size_block.rs) | Rust | -72 | -20 | -10 | -102 |
| [/home/eden/ProjectsRepo/my_ros/src/allocator/linked_list.rs](//home/eden/ProjectsRepo/my_ros/src/allocator/linked_list.rs) | Rust | -105 | -44 | -20 | -169 |
| [/home/eden/ProjectsRepo/my_ros/src/demo/concurrency.rs](//home/eden/ProjectsRepo/my_ros/src/demo/concurrency.rs) | Rust | -38 | 0 | -3 | -41 |
| [/home/eden/ProjectsRepo/my_ros/src/demo/cpu_exceptions.rs](//home/eden/ProjectsRepo/my_ros/src/demo/cpu_exceptions.rs) | Rust | -4 | 0 | -1 | -5 |
| [/home/eden/ProjectsRepo/my_ros/src/demo/double_fault.rs](//home/eden/ProjectsRepo/my_ros/src/demo/double_fault.rs) | Rust | -6 | 0 | -1 | -7 |
| [/home/eden/ProjectsRepo/my_ros/src/demo/heap_allocation.rs](//home/eden/ProjectsRepo/my_ros/src/demo/heap_allocation.rs) | Rust | -22 | 0 | -8 | -30 |
| [/home/eden/ProjectsRepo/my_ros/src/demo/memory.rs](//home/eden/ProjectsRepo/my_ros/src/demo/memory.rs) | Rust | -36 | -6 | -6 | -48 |
| [/home/eden/ProjectsRepo/my_ros/src/demo/mod.rs](//home/eden/ProjectsRepo/my_ros/src/demo/mod.rs) | Rust | -16 | 0 | -4 | -20 |
| [/home/eden/ProjectsRepo/my_ros/src/demo/println_eprintln.rs](//home/eden/ProjectsRepo/my_ros/src/demo/println_eprintln.rs) | Rust | -8 | 0 | -2 | -10 |
| [/home/eden/ProjectsRepo/my_ros/src/exit.rs](//home/eden/ProjectsRepo/my_ros/src/exit.rs) | Rust | -13 | 0 | -2 | -15 |
| [/home/eden/ProjectsRepo/my_ros/src/gdt.rs](//home/eden/ProjectsRepo/my_ros/src/gdt.rs) | Rust | -45 | 0 | -7 | -52 |
| [/home/eden/ProjectsRepo/my_ros/src/interrupts.rs](//home/eden/ProjectsRepo/my_ros/src/interrupts.rs) | Rust | -106 | -25 | -24 | -155 |
| [/home/eden/ProjectsRepo/my_ros/src/lib.rs](//home/eden/ProjectsRepo/my_ros/src/lib.rs) | Rust | -77 | -7 | -11 | -95 |
| [/home/eden/ProjectsRepo/my_ros/src/main.rs](//home/eden/ProjectsRepo/my_ros/src/main.rs) | Rust | -36 | -2 | -11 | -49 |
| [/home/eden/ProjectsRepo/my_ros/src/memory.rs](//home/eden/ProjectsRepo/my_ros/src/memory.rs) | Rust | -94 | -31 | -22 | -147 |
| [/home/eden/ProjectsRepo/my_ros/src/prelude.rs](//home/eden/ProjectsRepo/my_ros/src/prelude.rs) | Rust | 0 | 0 | -2 | -2 |
| [/home/eden/ProjectsRepo/my_ros/src/serial.rs](//home/eden/ProjectsRepo/my_ros/src/serial.rs) | Rust | -33 | -3 | -6 | -42 |
| [/home/eden/ProjectsRepo/my_ros/src/task/executor.rs](//home/eden/ProjectsRepo/my_ros/src/task/executor.rs) | Rust | -97 | -5 | -16 | -118 |
| [/home/eden/ProjectsRepo/my_ros/src/task/keyboard.rs](//home/eden/ProjectsRepo/my_ros/src/task/keyboard.rs) | Rust | -83 | -4 | -12 | -99 |
| [/home/eden/ProjectsRepo/my_ros/src/task/mod.rs](//home/eden/ProjectsRepo/my_ros/src/task/mod.rs) | Rust | -51 | 0 | -13 | -64 |
| [/home/eden/ProjectsRepo/my_ros/src/task/simple_executor.rs](//home/eden/ProjectsRepo/my_ros/src/task/simple_executor.rs) | Rust | -44 | 0 | -8 | -52 |
| [/home/eden/ProjectsRepo/my_ros/src/test_framework.rs](//home/eden/ProjectsRepo/my_ros/src/test_framework.rs) | Rust | -14 | 0 | -3 | -17 |
| [/home/eden/ProjectsRepo/my_ros/src/vga_buffer.rs](//home/eden/ProjectsRepo/my_ros/src/vga_buffer.rs) | Rust | -258 | -21 | -41 | -320 |
| [/home/eden/ProjectsRepo/my_ros/tests/heap_allocation.rs](//home/eden/ProjectsRepo/my_ros/tests/heap_allocation.rs) | Rust | -62 | -1 | -11 | -74 |
| [/home/eden/ProjectsRepo/my_ros/tests/should_panic.rs](//home/eden/ProjectsRepo/my_ros/tests/should_panic.rs) | Rust | -29 | 0 | -6 | -35 |
| [/home/eden/ProjectsRepo/my_ros/tests/stack_overflow.rs](//home/eden/ProjectsRepo/my_ros/tests/stack_overflow.rs) | Rust | -51 | -3 | -12 | -66 |

[Summary](results.md) / [Details](details.md) / [Diff Summary](diff.md) / Diff Details