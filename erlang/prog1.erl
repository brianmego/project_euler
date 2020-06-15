-module(prog1).
-export([solve/0]).

solve() ->
    L = lists:seq(1, 999),
    lists:sum([X || X <- L, divisible(X, 3) or divisible(X, 5)]).
divisible(X, Y) -> X rem Y == 0.
