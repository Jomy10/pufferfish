require 'pufferfish'
require 'htmlbeautifier'

# Executes CLI commands
class CommandExecutor
    def initialize(cmd)
        @cmd = cmd
    end

    def execute
        case @cmd.cmd
        when :parse
            file = @cmd.args[:file]
            output = Pufferfish::Parser::parse_file(file, @cmd.args[:base_dir])
            if @cmd.args[:pretty]
                HtmlBeautifier.beautify(output)
            else 
                output
            end
        end
    end
end