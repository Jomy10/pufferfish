require 'pufferfish'

Pufferfish::Builder.new(lambda { |b|
    b.html_dir = "html" # default: .
    b.template_dir = "templates" # default: .
    b.output_dir = "output" # default: .
    b.pretty = true # default: false
    b.minify = false # default: false
    b.verbose = true # default: false
})
