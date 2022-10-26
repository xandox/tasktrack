
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
            [CompletionResult]::new('current', 'current', [CompletionResultType]::ParameterValue, 'Show current active task')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List tasks')
            [CompletionResult]::new('new', 'new', [CompletionResultType]::ParameterValue, 'Create new task')
            [CompletionResult]::new('activate', 'activate', [CompletionResultType]::ParameterValue, 'Activate task')
            [CompletionResult]::new('edit', 'edit', [CompletionResultType]::ParameterValue, 'Edit task description')
            [CompletionResult]::new('report', 'report', [CompletionResultType]::ParameterValue, 'Generate report')
            [CompletionResult]::new('show', 'show', [CompletionResultType]::ParameterValue, 'Show task description')
            [CompletionResult]::new('add-range', 'add-range', [CompletionResultType]::ParameterValue, 'Manulay add task time range')
            [CompletionResult]::new('vacation-add', 'vacation-add', [CompletionResultType]::ParameterValue, 'Add vacation')
            [CompletionResult]::new('vacation-remove', 'vacation-remove', [CompletionResultType]::ParameterValue, 'Remove vacation')
            [CompletionResult]::new('vacation-list', 'vacation-list', [CompletionResultType]::ParameterValue, 'List vacations')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'tasktrack;current' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'tasktrack;list' {
            [CompletionResult]::new('-n', 'n', [CompletionResultType]::ParameterName, 'If set first *num_tasks*')
            [CompletionResult]::new('--num-tasks', 'num-tasks', [CompletionResultType]::ParameterName, 'If set first *num_tasks*')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'tasktrack;new' {
            [CompletionResult]::new('-u', 'u', [CompletionResultType]::ParameterName, 'Jira issue url')
            [CompletionResult]::new('--url', 'url', [CompletionResultType]::ParameterName, 'Jira issue url')
            [CompletionResult]::new('-t', 't', [CompletionResultType]::ParameterName, 'Some short text description')
            [CompletionResult]::new('--title', 'title', [CompletionResultType]::ParameterName, 'Some short text description')
            [CompletionResult]::new('-w', 'w', [CompletionResultType]::ParameterName, 'Workpackage')
            [CompletionResult]::new('--workpackage', 'workpackage', [CompletionResultType]::ParameterName, 'Workpackage')
            [CompletionResult]::new('-o', 'o', [CompletionResultType]::ParameterName, 'Objective')
            [CompletionResult]::new('--objective', 'objective', [CompletionResultType]::ParameterName, 'Objective')
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
            [CompletionResult]::new('-u', 'u', [CompletionResultType]::ParameterName, 'Set url to new value')
            [CompletionResult]::new('--url', 'url', [CompletionResultType]::ParameterName, 'Set url to new value')
            [CompletionResult]::new('-t', 't', [CompletionResultType]::ParameterName, 'Set title to new value')
            [CompletionResult]::new('--title', 'title', [CompletionResultType]::ParameterName, 'Set title to new value')
            [CompletionResult]::new('-w', 'w', [CompletionResultType]::ParameterName, 'Set workpackage to new value')
            [CompletionResult]::new('--workpackage', 'workpackage', [CompletionResultType]::ParameterName, 'Set workpackage to new value')
            [CompletionResult]::new('-o', 'o', [CompletionResultType]::ParameterName, 'Set objective to new value')
            [CompletionResult]::new('--objective', 'objective', [CompletionResultType]::ParameterName, 'Set objective to new value')
            [CompletionResult]::new('--drop-url', 'drop-url', [CompletionResultType]::ParameterName, 'Drop url value')
            [CompletionResult]::new('--drop-title', 'drop-title', [CompletionResultType]::ParameterName, 'Drop title value')
            [CompletionResult]::new('--drop-workpackage', 'drop-workpackage', [CompletionResultType]::ParameterName, 'Drop workpackage value')
            [CompletionResult]::new('--drop-objective', 'drop-objective', [CompletionResultType]::ParameterName, 'Drop objective value')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'tasktrack;report' {
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'If set print report in csv format')
            [CompletionResult]::new('--csv', 'csv', [CompletionResultType]::ParameterName, 'If set print report in csv format')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'tasktrack;show' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'tasktrack;add-range' {
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 'Date since generate report. Format %d-%m-%Y')
            [CompletionResult]::new('--since', 'since', [CompletionResultType]::ParameterName, 'Date since generate report. Format %d-%m-%Y')
            [CompletionResult]::new('-t', 't', [CompletionResultType]::ParameterName, 'Date till generate report. Format %d-%m-%Y')
            [CompletionResult]::new('--till', 'till', [CompletionResultType]::ParameterName, 'Date till generate report. Format %d-%m-%Y')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'tasktrack;vacation-add' {
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--since', 'since', [CompletionResultType]::ParameterName, 'since')
            [CompletionResult]::new('-t', 't', [CompletionResultType]::ParameterName, 't')
            [CompletionResult]::new('--till', 'till', [CompletionResultType]::ParameterName, 'till')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'tasktrack;vacation-remove' {
            [CompletionResult]::new('-i', 'i', [CompletionResultType]::ParameterName, 'i')
            [CompletionResult]::new('--id', 'id', [CompletionResultType]::ParameterName, 'id')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'tasktrack;vacation-list' {
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--since', 'since', [CompletionResultType]::ParameterName, 'since')
            [CompletionResult]::new('-t', 't', [CompletionResultType]::ParameterName, 't')
            [CompletionResult]::new('--till', 'till', [CompletionResultType]::ParameterName, 'till')
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
