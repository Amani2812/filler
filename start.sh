#!/bin/bash

if [[ $(uname -m) == "aarch64" ]]; then
  ../m1_game_engine -f ../maps/map02 -p1 target/debug/solution -p2 ../m1_robots/terminator
else
  ../linux_game_engine -f ../maps/map00 -p1 target/debug/solution -p2 ../linux_robots/terminator
fi