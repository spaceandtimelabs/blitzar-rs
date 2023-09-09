#!/bin/bash
set -eou pipefail

NEW_VERSION=$1

if ! [[ ${NEW_VERSION} =~ ^[0-9]+[.][0-9]+[.][0-9]+$ ]]
then
    echo "Incorrect version format: " $NEW_VERSION
    exit 1
fi

# configure rust lib to release
sed -i 's/version = "*.*.*" # This version will be automatically updated/version = "'${NEW_VERSION}'"/' Cargo.toml

FILES="benches/ examples/ src/ Cargo.toml README.md"

cargo publish --allow-dirty --token ${CRATES_TOKEN}

zip -r blitzar-v$NEW_VERSION.zip $FILES
tar -czvf blitzar-v$NEW_VERSION.tar.gz $FILES
