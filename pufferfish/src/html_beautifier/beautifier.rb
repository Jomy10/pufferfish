require "strscan"
# frozen_string_literal: true



module HtmlBeautifier
  class Parser
    def initialize
      @maps = []
      yield self if block_given?
    end

    def map(pattern, method)
      @maps << [pattern, method]
    end

    def scan(subject, receiver)
      @scanner = StringScanner.new(subject)
      dispatch(receiver) until @scanner.eos?
    end

    def source_so_far
      @scanner.string[0...@scanner.pos]
    end

    def source_line_number
      [source_so_far.chomp.split(%r{\n}).count, 1].max
    end

  private

    def dispatch(receiver)
      _, method = @maps.find { |pattern, _| @scanner.scan(pattern) }
      raise "Unmatched sequence" unless method

      receiver.__send__(method, *extract_params(@scanner))
    rescue StandardError => e
      raise "#{e.message} on line #{source_line_number}"
    end

    def extract_params(scanner)
      return [scanner[0]] unless scanner[1]

      params = []
      i = 1
      while scanner[i]
        params << scanner[i]
        i += 1
      end
      params
    end
  end
end
# frozen_string_literal: true

module HtmlBeautifier
  class RubyIndenter
    INDENT_KEYWORDS = %w[if elsif else unless while until begin for].freeze
    OUTDENT_KEYWORDS = %w[elsif else end].freeze
    RUBY_INDENT = %r{
      ^ ( #{INDENT_KEYWORDS.join("|")} )\b
      | \b ( do | \{ ) ( \s* \| [^|]+ \| )? $
    }xo
    RUBY_OUTDENT = %r{ ^ ( #{OUTDENT_KEYWORDS.join("|")} | \} ) \b }xo

    def outdent?(lines)
      lines.first =~ RUBY_OUTDENT
    end

    def indent?(lines)
      lines.last =~ RUBY_INDENT
    end
  end
end
# frozen_string_literal: true




module HtmlBeautifier
  class Builder
    DEFAULT_OPTIONS = {
      indent: "  ",
      initial_level: 0,
      stop_on_errors: false,
      keep_blank_lines: 0
    }.freeze

    def initialize(output, options = {})
      options = DEFAULT_OPTIONS.merge(options)
      @tab = options[:indent]
      @stop_on_errors = options[:stop_on_errors]
      @level = options[:initial_level]
      @keep_blank_lines = options[:keep_blank_lines]
      @new_line = false
      @empty = true
      @ie_cc_levels = []
      @output = output
      @embedded_indenter = RubyIndenter.new
    end

  private

    def error(text)
      return unless @stop_on_errors

      raise text
    end

    def indent
      @level += 1
    end

    def outdent
      error "Extraneous closing tag" if @level == 0
      @level = [@level - 1, 0].max
    end

    def emit(*strings)
      strings_join = strings.join("")
      @output << "\n" if @new_line && !@empty
      @output << (@tab * @level) if @new_line && !strings_join.strip.empty?
      @output << strings_join
      @new_line = false
      @empty = false
    end

    def new_line
      @new_line = true
    end

    def embed(opening, code, closing)
      lines = code.split(%r{\n}).map(&:strip)
      outdent if @embedded_indenter.outdent?(lines)
      emit opening, code, closing
      indent if @embedded_indenter.indent?(lines)
    end

    def foreign_block(opening, code, closing)
      emit opening
      emit_reindented_block_content code unless code.strip.empty?
      emit closing
    end

    def emit_reindented_block_content(code)
      lines = code.strip.split(%r{\n})
      indentation = foreign_block_indentation(code)

      indent
      new_line
      lines.each do |line|
        emit line.rstrip.sub(%r{^#{indentation}}, "")
        new_line
      end
      outdent
    end

    def foreign_block_indentation(code)
      code.split(%r{\n}).find { |ln| !ln.strip.empty? }[%r{^\s+}]
    end

    def preformatted_block(opening, content, closing)
      new_line
      emit opening, content, closing
      new_line
    end

    def standalone_element(elem)
      emit elem
      new_line if elem =~ %r{^<br[^\w]}
    end

    def close_element(elem)
      outdent
      emit elem
    end

    def close_block_element(elem)
      close_element elem
      new_line
    end

    def open_element(elem)
      emit elem
      indent
    end

    def open_block_element(elem)
      new_line
      open_element elem
    end

    def close_ie_cc(elem)
      if @ie_cc_levels.empty?
        error "Unclosed conditional comment"
      else
        @level = @ie_cc_levels.pop
      end
      emit elem
    end

    def open_ie_cc(elem)
      emit elem
      @ie_cc_levels.push @level
      indent
    end

    def new_lines(*content)
      blank_lines = content.first.scan(%r{\n}).count - 1
      blank_lines = [blank_lines, @keep_blank_lines].min
      @output << ("\n" * blank_lines)
      new_line
    end

    alias_method :text, :emit
  end
end
# frozen_string_literal: true



module HtmlBeautifier
  class Parser
    def initialize
      @maps = []
      yield self if block_given?
    end

    def map(pattern, method)
      @maps << [pattern, method]
    end

    def scan(subject, receiver)
      @scanner = StringScanner.new(subject)
      dispatch(receiver) until @scanner.eos?
    end

    def source_so_far
      @scanner.string[0...@scanner.pos]
    end

    def source_line_number
      [source_so_far.chomp.split(%r{\n}).count, 1].max
    end

  private

    def dispatch(receiver)
      _, method = @maps.find { |pattern, _| @scanner.scan(pattern) }
      raise "Unmatched sequence" unless method

      receiver.__send__(method, *extract_params(@scanner))
    rescue StandardError => e
      raise "#{e.message} on line #{source_line_number}"
    end

    def extract_params(scanner)
      return [scanner[0]] unless scanner[1]

      params = []
      i = 1
      while scanner[i]
        params << scanner[i]
        i += 1
      end
      params
    end
  end
end
# frozen_string_literal: true



module HtmlBeautifier
  class HtmlParser < Parser
    ELEMENT_CONTENT = %r{ (?:<%.*?%>|[^>])* }mx
    HTML_VOID_ELEMENTS = %r{(?:
      area | base | br | col | command | embed | hr | img | input | keygen |
      link | meta | param | source | track | wbr
    )}mix
    HTML_BLOCK_ELEMENTS = %r{(?:
      address | article | aside | audio | blockquote | canvas | dd | dir | div |
      dl | dt | fieldset | figcaption | figure | footer | form | h1 | h2 | h3 |
      h4 | h5 | h6 | header | hr | li | menu | noframes | noscript | ol | p |
      pre | section | table | tbody | td | tfoot | th | thead | tr | ul | video
    )}mix

    MAPPINGS = [
      [%r{(<%-?=?)(.*?)(-?%>)}om,
       :embed],
      [%r{<!--\[.*?\]>}om,
       :open_ie_cc],
      [%r{<!\[.*?\]-->}om,
       :close_ie_cc],
      [%r{<!--.*?-->}om,
       :standalone_element],
      [%r{<!.*?>}om,
       :standalone_element],
      [%r{(<script#{ELEMENT_CONTENT}>)(.*?)(</script>)}omi,
       :foreign_block],
      [%r{(<style#{ELEMENT_CONTENT}>)(.*?)(</style>)}omi,
       :foreign_block],
      [%r{(<pre#{ELEMENT_CONTENT}>)(.*?)(</pre>)}omi,
       :preformatted_block],
      [%r{(<textarea#{ELEMENT_CONTENT}>)(.*?)(</textarea>)}omi,
       :preformatted_block],
      [%r{<#{HTML_VOID_ELEMENTS}(?: #{ELEMENT_CONTENT})?/?>}om,
       :standalone_element],
      [%r{</#{HTML_BLOCK_ELEMENTS}>}om,
       :close_block_element],
      [%r{<#{HTML_BLOCK_ELEMENTS}(?: #{ELEMENT_CONTENT})?>}om,
       :open_block_element],
      [%r{</#{ELEMENT_CONTENT}>}om,
       :close_element],
      [%r{<#{ELEMENT_CONTENT}[^/]>}om,
       :open_element],
      [%r{<[\w\-]+(?: #{ELEMENT_CONTENT})?/>}om,
       :standalone_element],
      [%r{(\s*\r?\n\s*)+}om,
       :new_lines],
      [%r{[^<\n]+},
       :text]
    ].freeze

    def initialize
      super do |p|
        MAPPINGS.each do |regexp, method|
          p.map regexp, method
        end
      end
    end
  end
end
# frozen_string_literal: true

module HtmlBeautifier # :nodoc:
  module VERSION # :nodoc:
    MAJOR = 1
    MINOR = 4
    TINY  = 1

    STRING = [MAJOR, MINOR, TINY].join(".")
  end
end
# frozen_string_literal: true





module HtmlBeautifier
  #
  # Returns a beautified HTML/HTML+ERB document as a String.
  # html must be an object that responds to +#to_s+.
  #
  # Available options are:
  # tab_stops - an integer for the number of spaces to indent, default 2.
  # Deprecated: see indent.
  # indent - what to indent with ("  ", "\t" etc.), default "  "
  # stop_on_errors - raise an exception on a badly-formed document. Default
  # is false, i.e. continue to process the rest of the document.
  # initial_level - The entire output will be indented by this number of steps.
  # Default is 0.
  # keep_blank_lines - an integer for the number of consecutive empty lines
  # to keep in output.
  #
  def self.beautify(html, options = {})
    if options[:tab_stops]
      options[:indent] = " " * options[:tab_stops]
    end
    String.new.tap { |output|
      HtmlParser.new.scan html.to_s, Builder.new(output, options)
    }
  end
end
# Transform an html file to ruby
file_contents = ARGV.first
beautified_contents = HtmlBeautifier.beautify(file_contents)
puts beautified_contents