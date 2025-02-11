


place(X,Y, 1 ) :- square(Y,X), not covered(Y,X).
place(X,Y, 2 ) :- h(Y,X).
place(X,Y, 3 ) :- v(Y,X).
