predicate pre_original(a: array<int>,e: int,n:int)
  reads a
{
  (a.Length>0) && (e>0)
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
  ( 0<=n<=a.Length) && ( n==a.Length || a[n]==e) && (forall i::0<=i<n ==> e!=a[i])
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

