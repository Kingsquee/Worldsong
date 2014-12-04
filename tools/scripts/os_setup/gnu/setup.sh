# get current path
path="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )" &&



# Common

# Kernel
cd $path &&
echo "Generating the generator generator..." &&
cd ./../../compile_tools &&
mkdir -p ./target &&
rustc compile_tools.rs --out-dir ./target &&
echo "Running..." &&
cd ./target &&
./compile_tools
