predicate pre_original(a: array<int>,m: int)
  reads a
{
  ( a.Length >= 1)
}

predicate pre_gen(a: array<int>,m: int)
  reads a
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(a: array<int>,m: int)
  ensures pre_original(a,m ) <==> pre_gen(a,m )
{
}

predicate post_original(a: array<int>,m: int)
  requires pre_original(a,m)
  reads a
{
  ( forall k :: 0 <= k < a.Length ==> m >= a[k]) &&
  ( exists k :: 0 <= k < a.Length && m == a[k])
}

predicate post_gen(a: array<int>,m: int)
  requires pre_original(a,m)
  reads a
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(a: array<int>,m: int)
  requires pre_original(a,m )
  requires pre_gen(a,m )
  ensures post_original(a,m ) <==> post_gen(a,m )
{
}

