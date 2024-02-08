predicate pre_original(a:array<int>,b:array<int>,c:array<int>)
  reads a, b, c
{
  true
}

predicate pre_gen(a:array<int>,b:array<int>,c:array<int>)
  reads a, b, c
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(a:array<int>,b:array<int>,c:array<int>)
  ensures pre_original(a,b,c ) <==> pre_gen(a,b,c )
{
}

predicate post_original(a:array<int>,b:array<int>,c:array<int>)
  requires pre_original(a,b,c)
  reads a, b, c
{
  ( c.Length==b.Length+a.Length) && ( forall k :: 0 <= k < a.Length ==> c[k] == a[k]) && ( forall k :: 0 <= k < b.Length ==> c[k+a.Length] == b[k])
}

predicate post_gen(a:array<int>,b:array<int>,c:array<int>)
  requires pre_original(a,b,c)
  reads a, b, c
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(a:array<int>,b:array<int>,c:array<int>)
  requires pre_original(a,b,c )
  requires pre_gen(a,b,c )
  ensures post_original(a,b,c ) <==> post_gen(a,b,c )
{
}

