#!/bin/bash

if [[ $# -ne 2 ]] ; then 
  echo "Must supply 2 arguments"
  exit 1
fi

day=$1
name=$2
mkdir $day && cd $day

mkdir Python && cd Python && mkdir $name
cp ../../.pyTemplate.py $name/main.py

cd ..
mkdir Rust && cd Rust && cargo new $name
cp ../../.rustTemplate.rs $name/src/main.rs
cd $name && echo "/target
Cargo.lock" >> .gitignore

