View `tracing_subscriber` logs in `xilem`.

This program will present the contents of its own log file in a xilem app.

You have to run it twice to see any output - the first time it will generate the log file.

This program is already useful for surfacing xilem bugs and stress testing it.

Requires https://github.com/linebender/xilem/pull/917, after that PR is merged it can point at the xilem git.

# TODO

Loads of stuff to make this useful, including but not limited to...

 - Creating an index rather than grepping through all records every time the search string changes
 - Fix scroll area (not sure if it's me or a bug in `xilem`)
 - Fix crash when there are too many log lines (I'm more sure this is a `xilem` bug)