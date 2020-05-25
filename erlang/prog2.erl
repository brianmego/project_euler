-module(prog2).
-export([main/1]).

main([]) -> 
    R = fib(4000000),
    io:format("~w\n", [R]).

fib(Max) when is_integer(Max) -> fib([1, 1], Max).

fib([X,Y|T], Max) when X < Max -> fib([X + Y, X, Y] ++ T, Max);
fib([X,Y|T], _) -> sum_evens([X, Y|T]).

sum_evens(L) -> lists:sum([X || X <- L, X rem 2 == 0]).
