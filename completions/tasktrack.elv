
use builtin;
use str;

set edit:completion:arg-completer[tasktrack] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'tasktrack'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'tasktrack'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand current 'Show current active task'
            cand list 'List tasks'
            cand new 'Create new task'
            cand activate 'Activate task'
            cand edit 'Edit task description'
            cand report 'Generate report'
            cand show 'Show task description'
            cand add-range 'Manulay add task time range'
            cand vacation-add 'Add vacation'
            cand vacation-remove 'Remove vacation'
            cand vacation-list 'List vacations'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'tasktrack;current'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'tasktrack;list'= {
            cand -n 'If set first *num_tasks*'
            cand --num-tasks 'If set first *num_tasks*'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'tasktrack;new'= {
            cand -u 'Jira issue url'
            cand --url 'Jira issue url'
            cand -t 'Some short text description'
            cand --title 'Some short text description'
            cand -w 'Workpackage'
            cand --workpackage 'Workpackage'
            cand -o 'Objective'
            cand --objective 'Objective'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'tasktrack;activate'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'tasktrack;edit'= {
            cand -u 'Set url to new value'
            cand --url 'Set url to new value'
            cand -t 'Set title to new value'
            cand --title 'Set title to new value'
            cand -w 'Set workpackage to new value'
            cand --workpackage 'Set workpackage to new value'
            cand -o 'Set objective to new value'
            cand --objective 'Set objective to new value'
            cand --drop-url 'Drop url value'
            cand --drop-title 'Drop title value'
            cand --drop-workpackage 'Drop workpackage value'
            cand --drop-objective 'Drop objective value'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'tasktrack;report'= {
            cand -c 'If set print report in csv format'
            cand --csv 'If set print report in csv format'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'tasktrack;show'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'tasktrack;add-range'= {
            cand -s 'Date since generate report. Format %d-%m-%Y'
            cand --since 'Date since generate report. Format %d-%m-%Y'
            cand -t 'Date till generate report. Format %d-%m-%Y'
            cand --till 'Date till generate report. Format %d-%m-%Y'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'tasktrack;vacation-add'= {
            cand -s 's'
            cand --since 'since'
            cand -t 't'
            cand --till 'till'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'tasktrack;vacation-remove'= {
            cand -i 'i'
            cand --id 'id'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'tasktrack;vacation-list'= {
            cand -s 's'
            cand --since 'since'
            cand -t 't'
            cand --till 'till'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'tasktrack;help'= {
        }
    ]
    $completions[$command]
}
