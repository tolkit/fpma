#!/usr/bin/env bash

# don't want dependencies
cargo doc --no-deps --document-private-items
# remove old docs
rm -rf ./docs
# magic..?
echo "<meta http-equiv=\"refresh\" content=\"0; url=fpma\">" > target/doc/index.html
# copy to docs
cp -r target/doc ./docs
# copy a mito example to the dir
cp ./mitome.html ./docs/fpma