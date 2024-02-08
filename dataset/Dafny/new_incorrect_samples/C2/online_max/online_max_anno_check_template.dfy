predicate pre_original(a: array<int>,x: int,m:int,p:int)
  reads a
{
  ( 1<=x<a.Length) && ( a.Length!=0)
}

predicate pre_gen(a: array<int>,x: int,m:int,p:int)
  reads a
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(a: array<int>,x: int,m:int,p:int)
  ensures pre_original(a,x,m,p ) <==> pre_gen(a,x,m,p )
{
}

ghost predicate post_original(a: array<int>,x: int,m:int,p:int)
  requires pre_original(a,x,m,p)
  reads a
{
  (x<=p<a.Length)&&
  (x<=p<a.Length-1 ==> (forall i::0<=i<p ==> a[i]<a[p]))
}

ghost predicate post_gen(a: array<int>,x: int,m:int,p:int)
  requires pre_original(a,x,m,p)
  reads a
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(a: array<int>,x: int,m:int,p:int)
  requires pre_original(a,x,m,p )
  requires pre_gen(a,x,m,p )
  ensures post_original(a,x,m,p ) <==> post_gen(a,x,m,p )
{
}
