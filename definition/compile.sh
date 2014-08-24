#!/bin/bash
echo "Building orchestrator" && 
rustc -o ../orchestrator main.rs && 
echo "Building core utilities" && 
cd ../utils/definitions/core && 
./compile.sh && 
echo "Build complete!"