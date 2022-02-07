
Input = Struct.new(:cmd, :args)

# Parse CLI input
class CLI
    def initialize(input)
        @input = input
    end

    def parse
        @input.each_with_index do |var, idx|
            case var
            when "-p" || "--pretty"
                # Pretifies the html before returning
                @prettify = true
            when "-d" || "--base-directory"
                # sets the base directory for searching files
                # ! look out for absolute paths -> don't convert them
                @parse_next = :base_dir
            else
                if @parse_next == :base_dir 
                    @base = var
                    @parse_next = nil
                elsif @file.nil?
                    @file = var
                elsif @outputFile.nil?
                    @outputFile = var
                end
            end
        end

        vars = Hash.new
        vars[:file] = @file
        vars[:outputFile] = @outputFile
        vars[:base_dir] = @base
        vars[:pretty] = @prettify.nil? ? false : true
        Input.new(:parse, vars)
    end
end
