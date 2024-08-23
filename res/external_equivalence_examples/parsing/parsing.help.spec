definition[def_of_q]: forall X (q(X) <-> exists Y edge(X,Y)).
lemma(forward): q(5).
definition[def_of_g]: forall X ( g(X) <-> exists Y (q(X) and edge(Y,X)) ).
lemma: g(n) or q(n).
