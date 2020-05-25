-module(prog4).
-export([is_palindrome/1]).

is_palindrome(X) -> X =:= lists:reverse(X).
