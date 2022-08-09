complete -c tasktrack -n "__fish_use_subcommand" -s h -l help -d 'Print help information'
complete -c tasktrack -n "__fish_use_subcommand" -f -a "current" -d 'Show current active task'
complete -c tasktrack -n "__fish_use_subcommand" -f -a "list" -d 'List tasks'
complete -c tasktrack -n "__fish_use_subcommand" -f -a "new" -d 'Create new task'
complete -c tasktrack -n "__fish_use_subcommand" -f -a "activate" -d 'Activate task'
complete -c tasktrack -n "__fish_use_subcommand" -f -a "edit" -d 'Edit task description'
complete -c tasktrack -n "__fish_use_subcommand" -f -a "report" -d 'Generate report'
complete -c tasktrack -n "__fish_use_subcommand" -f -a "show" -d 'Show task description'
complete -c tasktrack -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c tasktrack -n "__fish_seen_subcommand_from current" -s h -l help -d 'Print help information'
complete -c tasktrack -n "__fish_seen_subcommand_from list" -s n -l num-tasks -d 'If set first *num_tasks*' -r
complete -c tasktrack -n "__fish_seen_subcommand_from list" -s h -l help -d 'Print help information'
complete -c tasktrack -n "__fish_seen_subcommand_from new" -s u -l url -d 'Jira issue url' -r
complete -c tasktrack -n "__fish_seen_subcommand_from new" -s t -l title -d 'Some short text description' -r
complete -c tasktrack -n "__fish_seen_subcommand_from new" -s w -l workpackage -d 'Workpackage' -r
complete -c tasktrack -n "__fish_seen_subcommand_from new" -s o -l objective -d 'Objective' -r
complete -c tasktrack -n "__fish_seen_subcommand_from new" -s h -l help -d 'Print help information'
complete -c tasktrack -n "__fish_seen_subcommand_from activate" -s h -l help -d 'Print help information'
complete -c tasktrack -n "__fish_seen_subcommand_from edit" -s u -l url -d 'Set url to new value' -r
complete -c tasktrack -n "__fish_seen_subcommand_from edit" -s t -l title -d 'Set title to new value' -r
complete -c tasktrack -n "__fish_seen_subcommand_from edit" -s w -l workpackage -d 'Set workpackage to new value' -r
complete -c tasktrack -n "__fish_seen_subcommand_from edit" -s o -l objective -d 'Set objective to new value' -r
complete -c tasktrack -n "__fish_seen_subcommand_from edit" -l drop-url -d 'Drop url value'
complete -c tasktrack -n "__fish_seen_subcommand_from edit" -l drop-title -d 'Drop title value'
complete -c tasktrack -n "__fish_seen_subcommand_from edit" -l drop-workpackage -d 'Drop workpackage value'
complete -c tasktrack -n "__fish_seen_subcommand_from edit" -l drop-objective -d 'Drop objective value'
complete -c tasktrack -n "__fish_seen_subcommand_from edit" -s h -l help -d 'Print help information'
complete -c tasktrack -n "__fish_seen_subcommand_from report" -s h -l help -d 'Print help information'
complete -c tasktrack -n "__fish_seen_subcommand_from show" -s h -l help -d 'Print help information'
