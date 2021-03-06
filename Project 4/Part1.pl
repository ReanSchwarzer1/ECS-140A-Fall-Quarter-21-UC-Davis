% -----------------  Ques-1 ----------------------
my_concat(L1,L2,L3):-
    make_concat(L1,L2,NewList),
    same(L3,NewList).

%concatanation of lists
make_concat([],L2,L2).
make_concat([H|T],L2,[H|L3]):- make_concat(T,L2,L3).

%checking if concatenated list and given list are same or not
same([], []).
same([H1|R1], [H2|R2]):-
    H1 = H2,
    same(R1, R2).




% -----------------  Ques-2 ----------------------
my_element_at(X,N,L):-
    nthelement(Y,L,N),
    X==Y.

nthelement(Y,[Y|_],1).
nthelement(Y,[_|L],K) :- nthelement(Y,L,K1), K is K1 + 1.



% --------------- Ques-3--------------------------

my_reverse(L1,L2):-
    list_rev(L1,ReversedList),
    ReversedList==L2.

list_concat([],L,L).
list_concat([X1|L1],L2,[X1|L3]) :- list_concat(L1,L2,L3).

list_rev([],[]).
list_rev([Head|Tail],Reversed) :-
   list_rev(Tail, RevTail),list_concat(RevTail, [Head],Reversed).


%----------------Ques-4----------------------------

my_flatten(L1,L2):-
    flatList(L1,FlattedList),
    FlattedList == L2.

flatList([],[]).
flatList([Head|Tail],R) :-
	flatList(Head,New_Head),
	flatList(Tail,New_Tail),
	append(New_Head,New_Tail,R).
flatList([Head|Tail1], [Head|Tail2]) :-
	Head \= [],
	Head \= [_|_],
  flatList(Tail1,Tail2).

%----------------Ques-5----------------------------

my_compress([H], [H]).
my_compress([H,H|T], X) :- my_compress([H|T],X).
my_compress([H|T], [H|X]) :- my_compress(T, X).








