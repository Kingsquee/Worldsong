#!/bin/bash
echo "" && 
echo "Building Dataset" &&
cargo build &&
echo "Done" &&
cd ../../orchestrator/src/ && 
./compile.sh