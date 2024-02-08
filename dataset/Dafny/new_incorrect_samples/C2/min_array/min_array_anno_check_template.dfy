predicate pre_original(a: array<int>,r:int)
  reads a
{
  ( a.Length > 0)
}

predicate pre_gen(a: array<int>,r:int)
  reads a
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(a: array<int>,r:int)
  ensures pre_original(a,r ) <==> pre_gen(a,r )
{
}

predicate post_original(a: array<int>,r:int)
  requires pre_original(a,r)
  reads a
{
  ( forall i :: 0 <= i < a.Length ==> r <= a[i]) }

predicate post_gen(a: array<int>,r:int)
  requires pre_original(a,r)
  reads a
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(a: array<int>,r:int)
  requires pre_original(a,r )
  requires pre_gen(a,r )
  ensures post_original(a,r ) <==> post_gen(a,r )
{
}

