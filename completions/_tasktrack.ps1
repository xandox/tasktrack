
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'tasktrack' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'tasktrack'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'tasktrack' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('current', 'current', [CompletionResultType]::ParameterValue, 'current')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'list')
            [CompletionResult]::new('new', 'new', [CompletionResultType]::ParameterValue, 'new')
            [CompletionResult]::new('activate', 'activate', [CompletionResultType]::ParameterValue, 'activate')
            [CompletionResult]::new('edit', 'edit', [CompletionResultType]::ParameterValue, 'edit')
            [CompletionResult]::new('report', 'report', [CompletionResultType]::ParameterValue, 'report')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'tasktrack;current' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'tasktrack;list' {
            [CompletionResult]::new('-n', 'n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--num-tasks', 'num-tasks', [CompletionResultType]::ParameterName, 'num-tasks')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'tasktrack;new' {
            [CompletionResult]::new('-u', 'u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--url', 'url', [CompletionResultType]::ParameterName, 'url')
            [CompletionResult]::new('-t', 't', [CompletionResultType]::ParameterName, 't')
            [CompletionResult]::new('--title', 'title', [CompletionResultType]::ParameterName, 'title')
            [CompletionResult]::new('-w', 'w', [CompletionResultType]::ParameterName, 'w')
            [CompletionResult]::new('--workpackage', 'workpackage', [CompletionResultType]::ParameterName, 'workpackage')
            [CompletionResult]::new('-o', 'o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--objective', 'objective', [CompletionResultType]::ParameterName, 'objective')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'tasktrack;activate' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'tasktrack;edit' {
            [CompletionResult]::new('-u', 'u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--url', 'url', [CompletionResultType]::ParameterName, 'url')
            [CompletionResult]::new('-t', 't', [CompletionResultType]::ParameterName, 't')
            [CompletionResult]::new('--title', 'title', [CompletionResultType]::ParameterName, 'title')
            [CompletionResult]::new('-w', 'w', [CompletionResultType]::ParameterName, 'w')
            [CompletionResult]::new('--workpackage', 'workpackage', [CompletionResultType]::ParameterName, 'workpackage')
            [CompletionResult]::new('-o', 'o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--objective', 'objective', [CompletionResultType]::ParameterName, 'objective')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'tasktrack;report' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'tasktrack;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
