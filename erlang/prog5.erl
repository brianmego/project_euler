-module(prog5).
-export([solve/0]).

solve() -> solve(20, 20).

solve(Attempt, Divisor) -> solve(Attempt, Divisor, Divisor).

solve(Attempt, Step, Divisor) when Attempt rem Divisor == 0 ->
    if 
        Divisor == 1 -> Attempt;
        Divisor > 1 -> solve(Attempt, Step, Divisor - 1)
    end;
solve(Attempt, Step, Divisor) when Attempt rem Divisor > 0 ->
    solve(Attempt + Step, Step, 20).
