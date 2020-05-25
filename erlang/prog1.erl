-module(prog1).
-export([main/1]).

main([]) -> 
    R = main(),
    io:format("~w\n", [R]).

main() ->
    L = lists:seq(1, 999),
    lists:sum([X || X <- L, divisible(X, 3) or divisible(X, 5)]).
divisible(X, Y) -> X rem Y == 0.
