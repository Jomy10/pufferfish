require_relative 'Pufferfish.rb'

GEM_NAME = PufferfishGem::GEM_NAME
GEM_VERSION = PufferfishGem::GEM_VERSION

task :default => :build

task :build do
  system "gem build " + GEM_NAME + ".gemspec"
end

task :install => :build do
  system "gem install " + GEM_NAME + "-" + GEM_VERSION + ".gem"
end

task :publish => :build do
  system 'gem push ' + GEM_NAME + "-" + GEM_VERSION + ".gem"
end

task :clean do
  system "rm *.gem"
end