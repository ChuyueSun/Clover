predicate pre_original<T>(a: array<T>,P: T->bool,n: int)
  reads a
{
  ( exists i :: 0 <= i < a.Length && P(a[i]))
}

predicate pre_gen<T>(a: array<T>,P: T->bool,n: int)
  reads a
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq<T>(a: array<T>,P: T->bool,n: int)
  ensures pre_original(a,P,n ) <==> pre_gen(a,P,n )
{
}

predicate post_original<T>(a: array<T>,P: T->bool,n: int)
  requires pre_original(a,P,n)
  reads a
{
  ( 0 <= n < a.Length) && ( P(a[n])) && (forall k :: 0 <= k < n ==> !P(a[k]))
}

predicate post_gen<T(0)>(a: array<T>,P: T->bool,n: int)
  requires pre_original(a,P,n)
  reads a
{
  true // (#POST) && ... (#POST)
}

lemma post_eq<T>(a: array<T>,P: T->bool,n: int)
  requires pre_original(a,P,n )
  requires pre_gen(a,P,n )
  ensures post_original(a,P,n ) <==> post_gen(a,P,n )
{
}

