predicate pre_original(a: array<int>,offset:int,b: array<int>)
  reads a, b
{
  ( 0<offset<a.Length)
}

predicate pre_gen(a: array<int>,offset:int,b: array<int>)
  reads a, b
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(a: array<int>,offset:int,b: array<int>)
  ensures pre_original(a,offset,b ) <==> pre_gen(a,offset,b )
{
}

predicate post_original(a: array<int>,offset:int,b: array<int>)
  requires pre_original(a,offset,b)
  reads a, b
{
  ( b.Length==a.Length) &&
  ( forall i ::0<=i<a.Length ==>  b[(i+offset)%a.Length]==a[i])
}

predicate post_gen(a: array<int>,offset:int,b: array<int>)
  requires pre_original(a,offset,b)
  reads a, b
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(a: array<int>,offset:int,b: array<int>)
  requires pre_original(a,offset,b )
  requires pre_gen(a,offset,b )
  ensures post_original(a,offset,b ) <==> post_gen(a,offset,b )
{
}

