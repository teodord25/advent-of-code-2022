#!/bin/bash

if [[ $# -ne 1 ]] ; then 
  echo "Must supply 1 argument"
  exit 1
fi

day=$1
mkdir $day && cd $day

mkdir Python
touch Python/input.txt
cp ../.pyTemplate.py Python/main.py

cargo new Rust
touch Rust/src/input.txt
cp ../.rustTemplate.rs Rust/src/main.rs
cd Rust && echo "/target
Cargo.lock
/src/input.txt" >> .gitignore

