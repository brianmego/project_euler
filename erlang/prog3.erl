-module(prog3).
-export([main/1]).

main([]) -> 
    R = get_prime_factor(600851475143),
    io:format("~w\n", [R]).


get_prime_factor(Max) -> get_prime_factor(2, Max).

get_prime_factor(Max, Max) -> Max;
get_prime_factor(CurrentTest, Max) when Max rem CurrentTest == 0 -> get_prime_factor(CurrentTest, Max div CurrentTest);
get_prime_factor(CurrentTest, Max) -> get_prime_factor(CurrentTest + 1, Max).
