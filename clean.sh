#!/bin/bash
echo "";
echo "Removing target directories";
find . -maxdepth 2 -name target -type d -delete
echo "Done";
echo "";
echo "Removing Cargo locks";
rm -f ./common/Cargo.lock;
echo "Done";