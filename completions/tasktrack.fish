complete -c tasktrack -n "__fish_use_subcommand" -s h -l help -d 'Print help information'
complete -c tasktrack -n "__fish_use_subcommand" -f -a "current"
complete -c tasktrack -n "__fish_use_subcommand" -f -a "list"
complete -c tasktrack -n "__fish_use_subcommand" -f -a "new"
complete -c tasktrack -n "__fish_use_subcommand" -f -a "activate"
complete -c tasktrack -n "__fish_use_subcommand" -f -a "edit"
complete -c tasktrack -n "__fish_use_subcommand" -f -a "report"
complete -c tasktrack -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c tasktrack -n "__fish_seen_subcommand_from current" -s h -l help -d 'Print help information'
complete -c tasktrack -n "__fish_seen_subcommand_from list" -s n -l num-tasks -r
complete -c tasktrack -n "__fish_seen_subcommand_from list" -s h -l help -d 'Print help information'
complete -c tasktrack -n "__fish_seen_subcommand_from new" -s u -l url -r
complete -c tasktrack -n "__fish_seen_subcommand_from new" -s t -l title -r
complete -c tasktrack -n "__fish_seen_subcommand_from new" -s w -l workpackage -r
complete -c tasktrack -n "__fish_seen_subcommand_from new" -s o -l objective -r
complete -c tasktrack -n "__fish_seen_subcommand_from new" -s h -l help -d 'Print help information'
complete -c tasktrack -n "__fish_seen_subcommand_from activate" -s h -l help -d 'Print help information'
complete -c tasktrack -n "__fish_seen_subcommand_from edit" -s u -l url -r
complete -c tasktrack -n "__fish_seen_subcommand_from edit" -s t -l title -r
complete -c tasktrack -n "__fish_seen_subcommand_from edit" -s w -l workpackage -r
complete -c tasktrack -n "__fish_seen_subcommand_from edit" -s o -l objective -r
complete -c tasktrack -n "__fish_seen_subcommand_from edit" -s h -l help -d 'Print help information'
complete -c tasktrack -n "__fish_seen_subcommand_from report" -s h -l help -d 'Print help information'
