
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
            cand current 'current'
            cand list 'list'
            cand new 'new'
            cand activate 'activate'
            cand edit 'edit'
            cand report 'report'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'tasktrack;current'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'tasktrack;list'= {
            cand -n 'n'
            cand --num-tasks 'num-tasks'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'tasktrack;new'= {
            cand -u 'u'
            cand --url 'url'
            cand -t 't'
            cand --title 'title'
            cand -w 'w'
            cand --workpackage 'workpackage'
            cand -o 'o'
            cand --objective 'objective'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'tasktrack;activate'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'tasktrack;edit'= {
            cand -u 'u'
            cand --url 'url'
            cand -t 't'
            cand --title 'title'
            cand -w 'w'
            cand --workpackage 'workpackage'
            cand -o 'o'
            cand --objective 'objective'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'tasktrack;report'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'tasktrack;help'= {
        }
    ]
    $completions[$command]
}
