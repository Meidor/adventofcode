run-today:
    cargo run --release
run-all:
    cargo run --release -- all
run-day day:
    cargo run --release -- {{day}}
run-part day part:
    cargo run --release -- {{day}} {{part}}
flamegraph year day part:
    CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph --root --bin aoc-{{year}} -o flamegraphs/{{day}}_{{part}}.svg -- {{day}} {{part}}
dhat-day day:
    CARGO_PROFILE_RELEASE_DEBUG=true cargo run --features dhat-heap {{day}}
dhat-part day part:
    CARGO_PROFILE_RELEASE_DEBUG=true cargo run --features dhat-heap {{day}} {{part}}
create:
    cargo run --package=setup
create-specific year day:
    cargo run --package=setup {{year}} {{day}}
setup year:
    cargo generate --path ./template-solutions -d year={{year}} --name aoc-{{year}}
bench:
    cargo bench
bench-day day:
    cargo bench -- {{day}}
bench-part day part:
    cargo bench -- {{day}} {{part}}
[linux]
pgo year:
    rm -rf /tmp/pgo-data
    RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data -Ctarget-cpu=native" cargo build --release --target=x86_64-unknown-linux-gnu
    ./target/x86_64-unknown-linux-gnu/release/aoc-{{year}} all
    ./target/x86_64-unknown-linux-gnu/release/aoc-{{year}} all
    ./target/x86_64-unknown-linux-gnu/release/aoc-{{year}} all
    ~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data
    RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata -Ctarget-cpu=native" cargo build --release --target=x86_64-unknown-linux-gnu
[linux]
run-pgo year:
    ./target/x86_64-unknown-linux-gnu/release/aoc-{{year}} all