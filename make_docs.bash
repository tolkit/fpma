#!/usr/bin/env bash

# don't want dependencies
cargo doc --no-deps --document-private-items
# remove old docs
rm -rf ./docs
# magic..?
echo "<meta http-equiv=\"refresh\" content=\"0; url=fpma\">" > target/doc/index.html
# copy to docs
cp -r target/doc ./docs
# run an example
# the assembled malus mitochondria, all in one.
# takes ~ 20 seconds.
./target/release/fpma \
--plant-mito mitome/malus.fasta \
--nhmmer-path /Users/mb39/bin/nhmmer \
--hmms-path ./fastas/angiosperm_hmms/ \
--plot mitome \
--e-value 0.0000000000001
# move it to the docs.
mv ./mitome.html ./docs/fpma