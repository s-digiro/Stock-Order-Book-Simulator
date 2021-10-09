# Order Book Programming Excercise
Simulates order book, buying, selling, canceling, and trading based on csv input.

	usage: order_book <path/to/input.csv> [-t]

	options:

	-t: Enables Trading. Without this switch, book crossing will trigger a
	    reject

## Input
Order Book takes input in the form of a file containing CSV.

An example is included in the project called input_file.csv.

To create place a new order, the format is

	N, <user_id>, <stock>, <price>, <amount>, <(B)uy/(S)ell>, <order_id>

The books can be flushed with

	F

## Output
Output is also in the form of CSV.

Any orders will be acknowledged in the form of

	A, <user_id>, <order_id>

Any time the top of book change, it will be published in the form of

	B, <(B)uy/(S)ell>, <price>, <amount>

If trading is not enabled, any asks that cross the top of book wil be rejectd. This will be shown in the form of

	R, <user_id>, <order_id>

If trading is enabled, trades will be shown in the form of

	T, <user_id>, <order_id>, <user_id>, <order_id>, <price>, <amount>

## Build
From within the project, run

	cargo build

## Run
From within the project, run

	cargo run <path/to/input.csv>
or

	order_book <path/to/input.csv>

There is an input.csv in the root folder of the project.

If you would like to run with trading enabled run

	cargo run <path/to/input.csv> -t

or

	order_book <path/to/input.csv> -t
