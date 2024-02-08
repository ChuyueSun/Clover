predicate pre_original<T(==)>(xs: seq<T>,a: array<T>)
  reads a
{
  true
}

predicate pre_gen<T(==)>(xs: seq<T>,a: array<T>)
  reads a
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq<T(==)>(xs: seq<T>,a: array<T>)
  ensures pre_original(xs,a ) <==> pre_gen(xs,a )
{
}

predicate post_original<T(==)>(xs: seq<T>,a: array<T>)
  requires pre_original(xs,a)
  reads a
{
  ( a.Length == |xs|) &&
  ( forall i :: 0 <= i < |xs| ==> a[i] == xs[i])
}

predicate post_gen<T(==)>(xs: seq<T>,a: array<T>)
  requires pre_original(xs,a)
  reads a
{
  true // (#POST) && ... (#POST)
}

lemma post_eq<T>(xs: seq<T>,a: array<T>)
  requires pre_original(xs,a )
  requires pre_gen(xs,a )
  ensures post_original(xs,a ) <==> post_gen(xs,a )
{
}

