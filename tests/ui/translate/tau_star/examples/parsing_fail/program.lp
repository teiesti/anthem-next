p(X) :- q(X).
p(X) :- q(X++1), not r(X).
q :- t.
