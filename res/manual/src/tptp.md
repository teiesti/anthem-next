# TPTP Problem (.p)

Anthem produces [TPTP]() problem files containing typed first-order formulas ([TFF]()).

## The Standard Preamble
The standard preamble (standard_interpretations.p) contains (problem-independent) types and axioms partially axiomatizing the conditions of [standard interpretations](https://doi.org/10.1017/S1471068420000344).

#### Types
The type declarations define the sorts of the target (formula representation) language (the numeral type corresponds to `vampire`'s built-in integer type, `$int`):
```
    tff(general_type, type, general: $tType).
    tff(symbol_type, type, symbol: $tType).
```

Objects within the universe of a subsort must be mapped to an object within the supersort universe:
```
    tff(f__integer___decl, type, f__integer__: ($int) > general).
    tff(f__symbolic___decl, type, f__symbolic__: (symbol) > general).
    tff(p__is_integer__decl, type, p__is_integer__: (general) > $o).
    tff(p__is_symbolic__decl, type, p__is_symbolic__: (general) > $o).
```

`#inf` and `#sup` are special general terms which do not belong to the symbol or numeral subsorts:
```
    tff(inf_type, type, c__infimum__: general).
    tff(sup_type, type, c__supremum__: general).
```

General terms are ordered:
```
    tff(p__less_equal__decl, type, p__less_equal__: (general * general) > $o).
    tff(p__less__decl, type, p__less__: (general * general) > $o).
    tff(p__greater_equal__decl, type, p__greater_equal__: (general * general) > $o).
    tff(p__greater__decl, type, p__greater__: (general * general) > $o).
```

#### Axioms

```
    tff(p__is_integer__def_ax, axiom, ![X: general]: (p__is_integer__(X) <=> (?[N: $int]: (X = f__integer__(N))))).
    tff(p__is_symbolic__def_ax, axiom, ![X1: general]: (p__is_symbolic__(X1) <=> (?[X2: symbol]: (X1 = f__symbolic__(X2))))).
```

The universe of general terms consists of symbols, numerals, `#inf`, and `#sup`:
```
    tff(general_universe_ax, axiom, ![X: general]: ((X = c__infimum__) | p__is_integer__(X) | p__is_symbolic__(X) | (X = c__supremum__))).
```

The mappings from subsorts to supersorts should preserve equality between terms:
```
    tff(f__integer__def_ax, axiom, ![N1: $int, N2: $int]: ((f__integer__(N1) = f__integer__(N2)) <=> (N1 = N2))).
    tff(f__symbolic__def_ax, axiom, ![S1: symbol, S2: symbol]: ((f__symbolic__(S1) = f__symbolic__(S2)) <=> (S1 = S2))).
```

Numerals are ordered analogously to integers:
```
    tff(numeral_ordering_ax, axiom, ![N1: $int, N2: $int]: (p__less_equal__(f__integer__(N1), f__integer__(N2)) <=> $lesseq(N1, N2))).
```

The ordering is transitive:
```
    tff(transitive_ordering_ax, axiom, ![X1: general, X2: general, X3: general]: ((p__less_equal__(X1, X2) & p__less_equal__(X2, X3)) => p__less_equal__(X1, X3))).
```

The ordering is total:
```
    tff(antisymmetric_ordering_ax, axiom, ![X1: general, X2: general]: ((p__less_equal__(X1, X2) & p__less_equal__(X2, X1)) => (X1 = X2))).
    tff(strongly_connected_ordering_ax, axiom, ![X1: general, X2: general]: (p__less_equal__(X1, X2) | p__less_equal__(X2, X1))).
```

The remaining relations are defined in terms of less or equal:
```
    tff(p__less__def_ax, axiom, ![X1: general, X2: general]: (p__less__(X1, X2) <=> (p__less_equal__(X1, X2) & (X1 != X2)))).
    tff(p__greater_equal__def_ax, axiom, ![X1: general, X2: general]: (p__greater_equal__(X1, X2) <=> p__less_equal__(X2, X1))).
    tff(p__greater__def_ax, axiom, ![X1: general, X2: general]: (p__greater__(X1, X2) <=> (p__less_equal__(X2, X1) & (X1 != X2)))).
```

`#inf` is the minimum general term, numerals are less than symbols, and `#sup` is the maximum general term:
```
    tff(minimal_element_ax, axiom, ![N: $int]: p__less__(c__infimum__, f__integer__(N))).
    tff(numerals_less_than_symbols_ax, axiom, ![N: $int, S: symbol]: p__less__(f__integer__(N), f__symbolic__(S))).
    tff(maximal_element_ax, axiom, ![S: symbol]: p__less__(f__symbolic__(S), c__supremum__)).
```

## Axioms Supporting External Equivalence
The standard preamble is part of every verification task.
Additional axioms are added to this partial axiomatization based on the problem at hand.

Let `P` denote the set of problem types consisting of function constants for each placeholder in the user guide.
Let `F` denote the set of symbolic constants (excluding placeholders) occurring anywhere in the problem.
To ensure that each `f` in `F` satisfies the unique name assumption of Herbrand interpretations, we need a set of axioms `O` defining a total order on `F`.
For instance, if `F` is `{a, b, c}`, then `O` is `{a < b, b < c}`.
Note that `a < c` is a consequence of the transitivity axiom of the preamble.
Additionally, we need a type declaration for every predicate in the problem (denote this set of declarations as `R`).
We extend the standard preamble with \\(P \cup F \cup O \cup R\\).
For example, for a problem containing an integer placeholder `k$`, symbolic constants `a` and `c`, and predicates `p/2` and `q/1`, we add the axioms
```
    tff(type, type, k$i: $int).
    tff(type, type, a: symbol).
    tff(type, type, c: symbol).
    tff(symbolic_constant_order, axiom, p__less__(f__symbolic__(a), f__symbolic__(c))).
    tff(predicate_0, type, p: (general * general) > $o).
    tff(predicate_1, type, q: (general) > $o).
```

## Axioms Supporting Strong Equivalence
Since strong equivalence does not support user guides or placeholders, the standard preamble is extended with \\(F \cup O \cup R\\) instead.
Additionally, we need axioms representing the ordering between the `here` and `there` worlds.
Thus, for a pair of predicates `(hp, tp)` we add the axiom
\\[\widetilde{\forall} (hp \rightarrow tp) \\]
