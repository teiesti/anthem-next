# Input File Format

### Writing Target Language Formulas

Terms are symbolic constants (`a`, `aB`, etc.), numerals (`1`, `-50`, etc.), variables (`V`, `Var$i`, etc.), zero-arity function constants (`a$g`, `n$i`, etc.), or `#inf` or `#sup`.
Terms composed of arithmetic operations `+, -, *` and integer-sorted operands are supported (`1 + 3`, `X$ - Y$`, etc.).
Atoms are predicate symbols followed by a tuple of terms (`p(1, X, V$)`, `q`, etc.).
Comparisons consist of a general term followed by one or more (relation, term) pairs (`a = b`, `0 <= N$ < 9`, etc.).
The relations
```
    =, !=, <, >, <=, =>
```
are supported.
The traditional binary connectives conjunction, disjunction, negation, implication, and equivalence are supported (written `and`, `or`, `not`, `->` and `<->`, respectively).
The reverse implication connective is defined as follows:
```
    F <- G
```
is understood as
```
    G -> F
```
The universal and existential quantifiers are written `forall` and `exists`.
Variables following a quantifier are separated by whitespace.
For example,
```
    forall X Y ( p(X, Y) <-> exists Z ( q(X,Y,Z) and X != Z) )
```


### Annotated Formulas
Specifications, user guides, and proof outlines contain annotated formulas.

An annotated formula is a first-order formula from the target language annotated with a role, and (optionally) with a name and/or direction. In general, an annotated formula is written
```
    role(direction)[name]: formula.
```

Valid roles are `assumption`, `spec`, `definition`, `lemma`, `inductive-lemma`.
Valid directions are `forward`, `backward`, `universal`.
In the task of proving the equivalence of a program `Π` to a specification `S`, we must derive
1. `Π` from `S` (the `forward` direction)
2. `S` from `Π` (the `backward` direction)

Thus, a formula annotated with the `universal` direction will be used in both directions.
Finally, names are alphanumeric strings.
Omitting a direction defaults to `universal`, whereas omitting a name defaults to `unnamed_formula`.
