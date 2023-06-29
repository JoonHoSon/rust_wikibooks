#!/bin/bash

target_folder="target/debug"

if [[ ! -d "$target_folder" ]]; then
	# create target folder
	mkdir -p "$target_folder"
fi

cp ./src/main.rs "$target_folder"

cargo run "$target_folder/main.rs"