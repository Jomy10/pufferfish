module Pufferfish
    # Parser
    module Parser
        # Parses html
        #
        # ### Params
        # file: String
        def self.parse_file(file, base_dir)
            contents = File.read(file)
            
            # ^\\ -> don't count escape characters \%
            # ? -> smallest possible match
            contents = contents.gsub(/%(.*?[^\\])%/) { |_|
                match = Regexp.last_match.captures
                file = match[0]
                file = file
                if file != ""
                    # Append file extension (TODO: set in config)
                    if not (file =~ /.*\..*/)
                        file.concat(".html")
                    end

                    # Parse base dir
                    if not base_dir.nil?
                        if file =~ /\A\/.*\z/ # ?
                            parse_file(file, base_dir)
                        elsif base_dir =~ /\A.*\/\z/
                            parse_file("#{base_dir}#{file}", base_dir)
                        else
                            parse_file("#{base_dir}/#{file}", base_dir)
                        end
                    else
                        parse_file(file, base_dir)
                    end
                else
                    STDERR.puts "found empty template identifier %%. If this is intential, consider escaping: \\%\\%"
                end
            }

            # Replace escape characters and return
            contents.gsub(/\\%/, "%")
        end
    end

    # Builder
    class Builder
        def initialize(fn)
            builder_info = PufferfishBuilderInfo.new
            answer = fn.call(builder_info)
            build(builder_info)
        end

        def build(info)
            Dir.glob("#{info.html_dir}/**/*") do |filename|
                if info.verbose
                    STDERR.puts "compiling #{filename}..."
                end
                # TODO: replace with ruby calls
                system "puf #{filename} #{info.output_dir}/#{filename.gsub(/\A\/?.*\//, "")} -d #{info.template_dir}" + (info.pretty ? " -p" : "")
                if info.minify
                    file_content = `html-minifier #{info.output_dir}/#{filename.gsub(/\A\/?.*\//, "")} #{info.minify_flags}`
                    File.write("#{info.output_dir}/#{filename.gsub(/\A\/?.*\//, "")}", file_content)
                end
            end
        end
    end

    class PufferfishBuilderInfo
        attr_accessor :html_dir, :template_dir, :output_dir, :pretty, :minify, :minify_flags, :verbose
        def initialize
            @html_dir = "."
            @template_dir = "."
            @output_dir = "."
            @pretty = false
            @minify = false
            @minify_flags = ""
            @verbose = false
        end
    end
end