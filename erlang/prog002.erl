-module(prog002).
-export([fib/1]).

fib(Max) when is_integer(Max) -> fib([1, 1], Max).

fib([X,Y|T], Max) when X < Max -> fib([X + Y, X, Y] ++ T, Max);
fib([X,Y|T], _) -> sum_evens([X, Y|T]).

sum_evens(L) -> lists:sum([X || X <- L, X rem 2 == 0]).
