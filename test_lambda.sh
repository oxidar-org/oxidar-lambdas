#!/bin/bash
get_path() {
    local paths=("/admin" "/echo" )
    path=${paths[$RANDOM % ${#paths[@]}]}

    echo "$path"
}

for ((i=1; i<=200; i++)); do
    user=$(awk -v seed=$RANDOM 'BEGIN {srand(seed)} {lines[++count] = $0} END {print lines[int(rand() * count) + 1]}' users.txt)
    email=$(echo $user | cut -d '|' -f1)
    token=$(echo $user | cut -d '|' -f2)

    cargo lambda invoke validate_jwt  --data-ascii "{ \"token\": \"$token\", \"path\": \"$(get_path)\" }"
done
