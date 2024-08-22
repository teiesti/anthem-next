# Input File Format

### Annotated Formulas
Specifications, user guides, and proof outlines contain annotated formulas.

An annotated formula is a first-order formula from the target language annotated with a role, and (optionally) with a name and/or direction. In general, an annotated formula is written

role(direction)[name]: formula.

Valid roles are assumption, spec, definition, lemma, inductive-lemma.

Valid directions are forward, backward, universal.

Names are alphanumeric strings.
