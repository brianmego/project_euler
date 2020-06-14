-module(prog4).
-export([main/1]).

main([]) -> R = check_descending(999 * 999),
            io:format("~w\n", [R]).

check_descending(X) ->
    case is_palindrome(integer_to_list(X)) andalso has_three_digit_factors(X) of
        true -> X;
        _Else -> check_descending(X - 1)
    end.


is_palindrome(X) -> X =:= lists:reverse(X).

has_three_digit_factors(X) -> has_three_digit_factors(X, 999, 100).

has_three_digit_factors(_, N1, N2) when N1 == N2 - 1 -> false;
has_three_digit_factors(X, N1, N2) when X rem N1 > 0 -> has_three_digit_factors(X, N1 - 1, N2);
has_three_digit_factors(X, N1, N2) when X rem N1 == 0 ->
    Quotient = X div N1,
    if 
       Quotient >= 100 andalso Quotient =< 999 -> true;
       Quotient < 100 orelse Quotient > 999 -> has_three_digit_factors(X, N1 - 1, N2)
    end.
