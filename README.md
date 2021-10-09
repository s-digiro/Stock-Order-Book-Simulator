Order Book Programming Excercise
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
Simulates order book, buying, selling, canceling, and trading based on csv input.

usage: order_book <path/to/input.csv> [-t]

option:
	-t: Enables Trading. Without this switch, book crossing will trigger a
	    reject

Build
~~~~~
From within the project, run
	'cargo build'

Run
~~~
From within the project, run
	'cargo run <path/to/input.csv>'

There is an input.csv in the root folder of the project.

If you would like to run with trading enabled run
	'cargo run <path/to/input.csv> -t'

Notes
~~~~~
I chose to implement trading, one of the optional objectives in the assignment.
It can be enabled with the '-t' switch. It must come after the path though,
otherwise cargo run will think that it is a switch for it, not a program
argument.

I chose not to use docker because it seemed redundant for such a small project
that builds and runs so easily, with no outside influences other than a csv
file.


Improvements
~~~~~~~~~~~~
If I had more time, I probably would have made changes to allow the -t switch to
occur anywhere in the command line command.

I also probably would have used docker, just to demonstrate my ability to use
it, but it was getting late and that seemed unimportant compared to finishing
the task overall.

The project could also use much more in the way of unit testing.