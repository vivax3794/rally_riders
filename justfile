run:
    mold -run cargo run --features fast_compile --color always

dev:
    mold -run cargo run --features dev --features fast_compile --color always

build:
    mold -run cargo run --release --features release

