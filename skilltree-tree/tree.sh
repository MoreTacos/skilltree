#!/bin/bash
# run binary
cargo r
# copy and remove old skills and tree
cp ./skills ./../skilltree-web/src/skills
cp ./tree.svg.hbs ./../skilltree-web/templates/tree.svg.hbs
