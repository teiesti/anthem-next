# Program (.lp)
A logic program `Π` must be written in the mini-gringo dialect.
It should not have any rule heads containing input symbols.
Comments (lines prefaced by a `%`) are allowed, but directives (e.g. `#show`) are not.

### The Graph Coloring Program

A simple logic program without arithmetic is the following encoding of the graph coloring problem, which can also be found in res/examples/external_equivalence/coloring/coloring.lp.

```
    {color(X,Z)} :- vertex(X), color(Z).
    :- color(X,Z1), color(X,Z2), Z1 != Z2.
    aux(X) :- vertex(X), color(Z), color(X,Z).
    :- vertex(X), not aux(X).
    :- edge(X,Y), color(X,Z), color(Y,Z).
```



### The Primes Programs

A challenging task for Anthem is verifying the external equivalence of the following logic program, `primes.1.lp`

```
    composite(I*J) :- I > 1, J > 1.
    prime(I) :- I = a..b, not composite(I).
```

to the program `primes.2.lp`

```
    sqrtb(M) :- M = 1..b, M * M <= b, (M+1)*(M+1) > b.
    composite(I*J) :- sqrtb(M), I = 2..M, J = 2..b.
    prime(I) :- I = a..b, not composite(I).
```

This example can be found in the res/examples/external_equivalence/primes/simple directory.
