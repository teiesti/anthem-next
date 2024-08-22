# Program (.lp)


### The Graph Coloring Program

A simple logic program without arithmetic is the following encoding of the graph coloring program

* {color(X,Z)} :- vertex(X), color(Z).
* :- color(X,Z1), color(X,Z2), Z1 != Z2.
* aux(X) :- vertex(X), color(Z), color(X,Z).
* :- vertex(X), not aux(X).
* :- edge(X,Y), color(X,Z), color(Y,Z).



### The Primes Programs

A challenging task for Anthem is verifying the external equivalence of the following logic program, `primes.1`

* composite(I*J) :- I > 1, J > 1.
* prime(I) :- I = a..b, not composite(I).

to the program `primes.2`

* sqrtb(M) :- M = 1..b, M * M <= b, (M+1)*(M+1) > b.
* composite(I*J) :- sqrtb(M), I = 2..M, J = 2..b.
* prime(I) :- I = a..b, not composite(I).
