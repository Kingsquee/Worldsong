#!/bin/bash
echo "" &&
echo "Compiling Worldsong Suite" &&
cd ./dataset/src/ &&
./compile.sh &&
cd ../../processes/srcs/Placeholder/ &&
./compile.sh &&
cd ../../../orchestrator/src/ &&
./compile.sh &&
echo "" &&
echo "Worldsong is ready to roll!"