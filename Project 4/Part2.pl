use_module(library(lists)).

queens(N,Q):-
    nqueens(N,Soln),
    same(Soln,Q).


same([], []).
same([V1|L1], [V2|L2]):-
    V1 = V2,
    same(L1, L2).


nqueens(N, Soln) :-
	length(Soln, N),
	queen(Soln, N).

ktoN(N,N,[N]) :-!.
ktoN(K,N,[K|Tail]) :-
    K < N, K1 is K+1, ktoN(K1, N, Tail).


queen([],_).

queen([Q|Listq],N) :-
    queen(Listq, N),
    ktoN(1,N,Pos),
    member(Q, Pos),
    soln_check(Q,Listq, 1).



soln_check(_,[], _).
soln_check(Q,[Q1|Listq],Diagonaldist) :-
	Q =\= Q1,
	Temp is abs(Q1-Q),
	Temp =\= Diagonaldist,
	Diagonaldist1 is Diagonaldist + 1,
	soln_check(Q,Listq,Diagonaldist1).
