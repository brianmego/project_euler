-module(prog7).
-export([solve/0, get_prime_index/1]).
-import(lists,[member/2, last/1, nth/2]).

solve() -> get_prime_index(10001).

get_prime_index(Target) -> build_prime_list([2, 3], Target).

build_prime_list(L, Target) when length(L) == Target -> nth(Target, L);
build_prime_list(L, Target) -> build_prime_list(L ++ [get_next_prime(L)], Target).

get_next_prime(L) -> get_next_prime(L, last(L)).

get_next_prime(L, X) ->
    IsPrime = is_prime(X, L),
    if
        IsPrime -> X;
        true -> get_next_prime(L, X + 2)
    end.


is_prime(_, []) -> true;
is_prime(X, [H|_]) when X rem H == 0 -> false;
is_prime(X, [_|T]) -> is_prime(X, T).
