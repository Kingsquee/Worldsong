#!/bin/bash
echo "" &&
echo "Compiling Worldsong Suite" &&
echo "" &&
echo "Creating target directories" &&
mkdir -p ./dataset/target &&
mkdir -p ./orchestrator/target &&
mkdir -p ./processes/targets && 
mkdir -p ./utils/target &&
echo "Done" &&
echo "" &&
cd ./dataset/src/ &&
./compile.sh &&
cd ../../processes/srcs/Placeholder/ &&
./compile.sh &&
cd ../../../orchestrator/src/ &&
./compile.sh &&
echo "" &&
echo "Worldsong is ready to roll!"