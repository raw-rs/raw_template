# trash_

Raw rust based library. 

To build with features:


## Cargo aliases

Aliases are defined in `.cargo/config.toml` for common workflows:

| Alias | Short | Description | Expands to |
|-------|-------|-------------|------------|
| `test` | `t` | Run tests with `cargo-nextest` | `nextest run --config-file .cargo/nextest.toml` |
| `coverage` | `c` | HTML coverage report | `llvm-cov nextest --config-file .cargo/nextest.toml --html` |
| `lcov` | `l` | LCOV coverage file | `llvm-cov nextest --config-file .cargo/nextest.toml --lcov --output-path lcov.info` |
| `flamegraph` | `f` | Generate flamegraph | `flamegraph --bench` |

Prerequisites: Install `cargo-nextest`, `cargo-llvm-cov` (with LLVM), and `flamegraph`. Assumes `.cargo/nextest.toml` exists.



## How to use

1. Clone or fork this repository.
2. Add your library modules under `src/` (replacing the minimal scaffold).
3. Build with:

```powershell
cargo build
```

4. Run tests:

```powershell
cargo test
```

5. Generate documentation with the custom logo:

```powershell
cargo doc --open
```

## CI and Linting

The project uses GitHub Actions for continuous integration, running on both stable and nightly Rust toolchains. CI performs:

- Code formatting checks (`cargo fmt -- --check`)
- Linting with Clippy (`cargo clippy --all-targets --all-features -- -D warnings`) — warnings cause failures
- Tests across all targets and features

To run locally:

```bash
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

## Notes

- The crate name is `trash_` and the library is exposed as `raw_` in Cargo.
- Targets Rust edition 2024 with minimum supported version 1.92.
- Documentation includes a custom logo and detailed feature explanations.
- The original opening description and project scaffolding were copied from `trash_parallelism`.

## Flamegraph / bench profile

This project includes a bench profile and a convenient cargo alias for generating flamegraphs with `cargo-flamegraph` (the `flamegraph` tool). The alias is defined in `.cargo/config.toml` as `f` and expands to `flamegraph --release --bench`.

How to use

- Install the flamegraph tool (this repo uses the `flamegraph` crate):

```powershell
cargo install flamegraph
```

- Run the flamegraph alias from the workspace root to profile your benchmarks (release build with bench target):

```powershell
cargo f
```

What the bench profile does

- The `[profile.bench]` section in `Cargo.toml` enables optimizations (`opt-level = 3`) while keeping debug symbols (`debug = true`) so the generated flamegraph has useful symbol names and call stacks. This balances performance with symbol quality for accurate profiling.

Notes and platform specifics

- `cargo-flamegraph` uses `perf` on Linux and `dtrace` on other platforms; on Windows support may be limited. If you're on Linux and want to run perf without root, you may need to lower `perf_event_paranoid` (see the `cargo-flamegraph` README for details).
- If your benchmarks are under a particular bench target, `cargo f` will profile the default bench runners. You can also run `cargo flamegraph --bench your_bench_name` directly if you need to target a specific benchmark.

