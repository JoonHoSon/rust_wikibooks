#!/bin/bash

target_folder="target/debug"

if [[ ! -d "$target_folder" ]]; then
	mkdir -p "$target_folder"
fi

cp ./dict.txt "$target_folder"

cargo run "$target_folder/dict.txt" "supervisor"