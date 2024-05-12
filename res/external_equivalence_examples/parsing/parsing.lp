aux(X) :- edge(X,n), not t.
reachable(Z,V) :- edge(Z,Var), edge(Var, V), aux(Var).
