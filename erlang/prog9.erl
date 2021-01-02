-module(prog9).
-export([solve/0]).
-import(lists,[sublist/3]).

solve() -> get_triplet(1000).

get_triplet(X) -> true.

is_pythagorian(A, B, C) when (A*A) + (B*B) == (C*C) -> true;
is_pythagorian(_, _, _) -> false.
