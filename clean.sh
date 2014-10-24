#!/bin/bash
echo "";
echo "Removing target directories";
find . -maxdepth 3 -name target -type d | xargs rm -rf
echo "Done";
echo "";
echo "Removing Cargo locks";
rm -f ./common/Cargo.lock;
echo "Done";