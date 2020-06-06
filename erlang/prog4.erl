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

has_three_digit_factors(_, CurrentTest, Smallest) when CurrentTest == Smallest - 1 -> false;
has_three_digit_factors(X, CurrentTest, Smallest) when X rem CurrentTest > 0 -> has_three_digit_factors(X, CurrentTest - 1, Smallest); 
has_three_digit_factors(X, CurrentTest, Smallest) when X rem CurrentTest == 0 -> 
    Quotient = X div CurrentTest,
    if 
       Quotient >= 100 andalso Quotient =< 999 -> true;
       Quotient < 100 orelse Quotient > 999 -> has_three_digit_factors(X, CurrentTest - 1, Smallest)
    end.
