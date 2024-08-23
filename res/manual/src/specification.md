# Specification (.spec)
The verification of external equivalence compares a mini-gringo program to a specification.
A specification can be written as another mini-gringo program `Π` or as a control language specification `S`.

### Logic Program Specifications
If the specification is a program `Π`, then `Π` must not contain input symbols in any rule heads.
Additionally, `Π` must be tight and free of private recursion.
This is because the formula representation of `Π` will be obtained via tau-star and completion (`COMP[τ*Π1]`).
The completed definitions of private predicates from this theory will be treated as assumptions in the `forward` and `backward` directions of the proof.
The remaining formulas from `COMP[τ*Π1]` will be treated analogously to formulas with the `spec(universal)` annotation (as described below).


### Control Language Specifications
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

Specs with the `universal` annotation are treated as axioms in the `forward` direction of the proof, and as conjectures in the `backward` direction.
A spec with a `forward` annotation is an axiom in the `forward` direction and ignored in the backward direction.
Similarly, a spec with a `backward` annotation is a conjecture in the `backward` direction and ignored in the forward direction.
