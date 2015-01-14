# get current path
path="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )" &&

cd $path &&
echo "Compiling tools..." &&
cd ./../../compile_tools &&
mkdir -p ./target &&
cargo build &&
echo "Running..." &&
cargo run -- --all
