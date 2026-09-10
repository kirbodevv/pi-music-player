[ ! -f .env ] || export $(grep -v '^#' .env | xargs)
cargo zigbuild --no-default-features -F raspberry,debug --release --target aarch64-unknown-linux-gnu
scp target/aarch64-unknown-linux-gnu/release/music-player ${PI_USERNAME}@${PI_HOSTNAME}:~/music-player
