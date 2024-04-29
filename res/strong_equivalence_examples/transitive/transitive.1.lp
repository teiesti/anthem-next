% Origin: Infinitary equilibrium logic and strongly equivalent logic programs
{q(X, Y)} :- p(X), p(Y).
q(X, Z) :- q(X, Y), q(Y, Z), p(X), p(Y), p(Z).