#!/bin/bash

# check if the script is executed from the parent directory of where it is located
script_dir=$(dirname "$(realpath "$0")")
if [ "$(dirname "$script_dir")" != "$(pwd)" ]; then
    echo "Please execute the script from the parent directory of where it is located."
    exit 1
fi

# get the list of all Dockerfiles from the Dockerfiles directory
dockerfiles=$(find "$script_dir/Dockerfiles-os" -type f -name "Dockerfile*")

# get the end of each Dockerfile name (e.g., Dockerfile.ubuntu -> ubuntu)
os_names=$(echo "$dockerfiles" | sed -n 's/.*Dockerfile\.\(.*\)/\1/p')

# build the rust project
cargo build --release

# build all Docker images
for os_name in $os_names; do
    docker build -t "test_on_all_os:$os_name" -f "$script_dir/Dockerfiles-os/Dockerfile.$os_name" .
done

echo ""

# run all docker containers and say the OS name
for os_name in $os_names; do
    echo ""
    echo "===== test from: $os_name ====="
    docker run -i --rm "test_on_all_os:$os_name" $@
done

echo ""

# remove all Docker images
docker rmi $(docker images -q test_on_all_os)

