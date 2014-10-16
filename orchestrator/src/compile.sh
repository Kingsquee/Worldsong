#!/bin/bash
echo "" &&
echo "Building Orchestrator" && 
rustc -o ../target/orchestrator -L ../../dataset/target -L ../../dataset/target/deps -L ../../dataset/target/native/* main.rs &&
echo "Done"