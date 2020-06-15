-module(prog6).
-export([solve/0]).
-import(lists,[sum/1]).

solve() -> solve(100).

solve(X) -> square_of_sums(X) - sum_of_squares(X).

square_of_sums(X) -> square_of_sums(X - 1, X).

square_of_sums(0, Accum) -> Accum * Accum;
square_of_sums(X, Accum) -> square_of_sums(X - 1, Accum + X).

sum_of_squares(X) -> sum_of_squares(X - 1, [X * X]).

sum_of_squares(0, L) -> sum(L);
sum_of_squares(X, L) -> sum_of_squares(X - 1, L ++ [X*X]).
