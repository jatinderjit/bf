_default:
    @just run "./examples/1-to-5.bf"

run PROGRAM:
    cargo run -- "{{ PROGRAM }}"
