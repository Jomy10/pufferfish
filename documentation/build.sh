#!/usr/bin/env zsh

npm run build

rm -r ../docs
cp -a build/. ../docs