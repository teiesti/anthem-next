# analyze

The `analyze` command lets users assess properties of their program through its predicate dependency graph.
A predicate dependency graph for a program `Π` has all predicates occurring in `Π` as vertices, and an edge from `p` to `q` if `p` depends on `q`; that is, if `Π` contains a rule of one of the following forms
```
     p  :- ..., q, ...
    {p} :- ..., q, ...
```
An edge `pq` is positive if `q` is not negated nor doubly negated.


## Tightness
A program is tight if its predicate dependency graph has no cycles consisting of positive edges.
External equivalence can only be verified automatically if the program(s) are tight.
Anthem checks this condition automatically when `verify` is invoked.
Users can check their programs for tightness with the command
```
    anthem analyze program.lp --property tightness
```

## Private Recursion
A program contains private recursion with respect to a user guide if
* its predicate dependency graph has a cycle such that every vertex in it is a private symbol or
* it includes a choice rule with a private symbol in the head.
When verifying external equivalence, any logic program is subjected to a test for private recursion and rejected if it occurs.
