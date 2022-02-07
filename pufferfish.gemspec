require_relative 'Pufferfish.rb'

Gem::Specification.new do |s|
    s.name = PufferfishGem::GEM_NAME
    s.version = PufferfishGem::GEM_VERSION
    s.platform = Gem::Platform::RUBY
    s.summary = "An html templating engine that generates static html."
    s.description = "Pufferfish is a templating language for html. It generates raw html, meaning that it will not affect load times."
    s.authors = ["Jonas everaert"]
    s.email = ["info@jonaseveraert.be"]
    s.homepage = "https://github.com/jomy10/pufferfish"
    s.add_runtime_dependency "htmlbeautifier", "~> 1.4.1"
    s.license = "MIT"
    s.files = Dir.glob("{lib,bin}/**/*")
    s.require_path = "lib"
    s.executables = ["puf"]
end