#!/bin/bash
# get current path
path="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )" &&

cd $path/kernel/target &&
LD_LIBRARY_PATH=$LD_LIBRARY_PATH:./../../common/target:./../../commonset/target/deps:./../../commonset/target/native/* ./kernel
