# get current path
path="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )" &&

cd $path &&
echo "Cleaning project" &&
cd ./../.. &&
find . -name Cargo.lock -type f -delete &&
cd compile_tools &&
echo "Compiling tools..." &&
mkdir -p ./target &&
cargo build &&
echo "Running..." &&
cargo run -- --all
