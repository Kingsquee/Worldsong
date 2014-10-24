#!/bin/bash
cd ${0%/*}/kernel/target &&
LD_LIBRARY_PATH=$LD_LIBRARY_PATH:./../../common/target:./../../commonset/target/deps:./../../commonset/target/native/* ./kernel