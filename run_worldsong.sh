#!/bin/bash
cd ${0%/*}/orchestrator/target &&
LD_LIBRARY_PATH=$LD_LIBRARY_PATH:./../../dataset/target:./../../dataset/target/deps:./../../dataset/target/native/* ./orchestrator
