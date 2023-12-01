run-today:
    cargo run --release
run-all:
    cargo run --release -- all
run-part day part:
    cargo run --release -- {{day}} {{part}}
flamegraph day part:
    CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph --root --bin adventofcode -o flamegraphs/{{day}}_{{part}}.svg -- {{day}} {{part}}
dhat day part:
    CARGO_PROFILE_RELEASE_DEBUG=true cargo run --features dhat-heap {{day}} {{part}}
create:
    cargo run --bin=setup