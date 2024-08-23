# Specification (.spec)

A specification can be written as a mini-gringo program `Π` or as a control language specification `S`.
If the specification is a program `Π`, then `Π` must not contain input symbols in any rule heads.
Additionally, `Π` must be tight and free of private recursion.

If the specification is `S`, it consists of annotated formulas of two types: assumptions and specs.
Both types of formulas must be closed.
Furthermore, atoms within assumptions should only have input predicate symbols.
The following is a specification defining the expected behavior of the Graph Coloring program:

```
    assumption: forall X Y (edge(X,Y) -> vertex(X) and vertex(Y)).
    spec: forall X Z (color(X,Z) -> vertex(X) and color(Z)).
    spec: forall X (vertex(X) -> exists Z color(X,Z)).
    spec: forall X Z1 Z2 (color(X,Z1) and color(X,Z2) -> Z1 = Z2).
    spec: not exists X Y Z (edge(X,Y) and color(X,Z) and color(Y,Z)).
```
