#!/usr/bin/env zsh
# Build NPM package

cd ../gem
rake build

cp pufferfish-*.gem ../npm/pufferfish.gem
cd ../npm

gem install pufferfish.gem --install-dir ./gem

# npm publish
