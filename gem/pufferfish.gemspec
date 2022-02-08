require_relative 'Pufferfish.rb'

Gem::Specification.new do |s|
    s.name = PufferfishGem::GEM_NAME
    s.version = PufferfishGem::GEM_VERSION
    s.platform = Gem::Platform::RUBY
    s.required_ruby_version = "2.6.0"
    s.summary = "An extensible html templating engine that generates static html."
    s.description = "Pufferfish is an extensible html templating engine that generates raw html, meaning that it will not affect load times of websites.\n\nA full-blown javascript framework is sometimes a bit overkill for a static website. Pufferfish adds some simple templating to html so you don't have to use such a framework for small projects or for pages that require fast loading. Pufferfish will compile your files to raw html.\n\nFor more information on its usage, see Pufferfish's GitHub page."
    s.authors = ["Jonas Everaert"]
    s.email = ["info@jonaseveraert.be"]
    s.homepage = "https://github.com/jomy10/pufferfish"
    s.add_runtime_dependency "htmlbeautifier", "~> 1.4.1"
    s.license = "MIT"
    s.files = Dir.glob("{lib,bin}/**/*")
    s.require_path = "lib"
    s.executables = ["puf"]
end
