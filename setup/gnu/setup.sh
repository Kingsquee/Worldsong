# get current path
path="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )" &&

cd $path &&
pwd &&
echo "Cleaning projects" &&
cd ./../.. &&
pwd &&
find . -name Cargo.lock -type f -delete &&
find . -name target -type d -exec rm -rf {} \; &&

cd $path &&
pwd &&
echo "Compiling tools..." &&
cd ./../compile_tools &&
pwd &&
cargo clean &&
cargo build &&

echo "Running..." &&
cargo run -- --build-apps
