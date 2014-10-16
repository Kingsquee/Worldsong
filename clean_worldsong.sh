#!/bin/bash
echo "" &&
echo "Removing target directories" &&
rm -rf ./dataset/target &&
rm -rf ./orchestrator/target &&
rm -rf ./processes/targets && 
rm -rf ./utils/target &&
echo "Done" && 
echo "" &&
echo "Removing Cargo locks" &&
rm -f ./dataset/Cargo.lock &&
rm -f ./utils/Cargo.lock