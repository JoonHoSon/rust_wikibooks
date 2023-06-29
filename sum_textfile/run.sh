#!/bin/bash

target_folder="target/debug"

if [[ ! -d "$target_folder" ]]; then
	# create target folder
	mkdir -p "$target_folder"
fi

cp ./text1.txt "$target_folder"
cp ./text2.txt "$target_folder"

cargo run "$target_folder/text1.txt" "$target_folder/text2.txt" "$target_folder/text3.txt"