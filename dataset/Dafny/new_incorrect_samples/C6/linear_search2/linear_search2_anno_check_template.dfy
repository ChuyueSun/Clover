predicate pre_original(a: array<int>,e: int,n:int)
  reads a
{
  exists i::0<=i<a.Length && ( a[i]==e)
}

predicate pre_gen(a: array<int>,e: int,n:int)
  reads a
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(a: array<int>,e: int,n:int)
  ensures pre_original(a,e,n ) <==> pre_gen(a,e,n )
{
}

predicate post_original(a: array<int>,e: int,n:int)
  requires pre_original(a,e,n)
  reads a
{
  ( 0<=n<a.Length) && ( a[n]==e) && (forall k :: 0 <= k < n ==> a[k] != e)
}

predicate post_gen(a: array<int>,e: int,n:int)
  requires pre_original(a,e,n)
  reads a
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(a: array<int>,e: int,n:int)
  requires pre_original(a,e,n )
  requires pre_gen(a,e,n )
  ensures post_original(a,e,n ) <==> post_gen(a,e,n )
{
}

