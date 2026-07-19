cr() {
    cargo run --bin "$1"
}

crr() {
    cargo run --release --bin "$1"
}

cri() {
    cargo run --bin "$1" < io/input.txt
}

crri() {
    cargo run --release --bin "$1" < io/input.txt
}

cro() {
    cargo run --bin "$1" > io/output.txt
}

crro() {
    cargo run --release --bin "$1" > io/output.txt
}

crio() {
    cargo run --bin "$1" < io/input.txt > io/output.txt
}

crrio() {
    cargo run --release --bin "$1" < io/input.txt > io/output.txt
}

cre() {
    cargo run --example "$1"
}

sub() {
    cargo run --bin submit -- "src/bin/$1.rs"
}

sube() {
    cargo run --bin submit -- "examples/$1.rs"
}

init() {
    cargo run --bin init -- "$@"
}