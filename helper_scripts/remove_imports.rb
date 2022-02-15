#!/usr/bin/env ruby
# simle cat program that removes require statements in ruby
file = ARGV[0]
contents = File.read(file)
contents = contents.gsub(/require ["'].*["']/, "")
puts contents