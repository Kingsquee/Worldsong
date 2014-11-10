#!/bin/bash
# get current path
path="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )" &&
cd $path &&
echo "" &&
echo "Compiling Worldsong Suite" &&
echo "" &&

# compiling common
cd ./common/src/ &&
./compile.sh &&

cd $path &&

# compiling kernel
cd ./kernel/src/ &&
./compile.sh &&

cd $path &&

echo "" &&
echo "Compiling Processes" &&
cd ./processes/ &&
find ./*/ -name "compile.sh" -type f -exec {} \;
echo "" &&
echo "Worldsong Compiled Successfully!"
