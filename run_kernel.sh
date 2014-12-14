#!/bin/bash
# get current path
path="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )" &&

cd $path/kernel/target &&
LD_LIBRARY_PATH=$LD_LIBRARY_PATH:./../../common/target:./../../common/target/deps:./../../common/target/native/*:./../../schedules/variable_update/schedule/target:./../../schedules/fixed_update/schedule/target:./../../schedules/pause/schedule/target:./../../schedules/fixed_update/processes/graphics/target:./../../schedules/variable_update/processes/input/target ./kernel
