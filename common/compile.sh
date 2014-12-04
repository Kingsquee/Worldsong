#!/bin/bash
# get current path
path="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )" &&

# modifying Common affects everything in the framework, so ensure the entire framework is recompiled.
cd $path/../ &&
./compile.sh
