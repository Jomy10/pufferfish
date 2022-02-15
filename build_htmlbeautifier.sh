#!/usr/bin/env zsh

# Builds the beautifier.rb script in pufferfish/src/html_beautifier/beautifier.rb

RESULT_FILE="pufferfish/src/html_beautifier/beautifier.rb"
touch $RESULT_FILE
echo "require \"strscan\"" > $RESULT_FILE
./helper_scripts/remove_imports.rb "html_beautifier/htmlbeautifier/lib/htmlbeautifier/parser.rb" >> $RESULT_FILE
./helper_scripts/remove_imports.rb "html_beautifier/htmlbeautifier/lib/htmlbeautifier/ruby_indenter.rb" >> $RESULT_FILE
./helper_scripts/remove_imports.rb "html_beautifier/htmlbeautifier/lib/htmlbeautifier/builder.rb" >> $RESULT_FILE
./helper_scripts/remove_imports.rb "html_beautifier/htmlbeautifier/lib/htmlbeautifier/parser.rb" >> $RESULT_FILE
./helper_scripts/remove_imports.rb "html_beautifier/htmlbeautifier/lib/htmlbeautifier/html_parser.rb" >> $RESULT_FILE
./helper_scripts/remove_imports.rb "html_beautifier/htmlbeautifier/lib/htmlbeautifier/version.rb" >> $RESULT_FILE
./helper_scripts/remove_imports.rb "html_beautifier/htmlbeautifier/lib/htmlbeautifier.rb" >> $RESULT_FILE
cat "html_beautifier/execute.rb" >> $RESULT_FILE
