predicate pre_original(a:array<int>,c:array<int>)
  reads a, c
{
  ( a.Length>0)
}

predicate pre_gen(a:array<int>,c:array<int>)
  reads a, c
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(a:array<int>,c:array<int>)
  ensures pre_original(a,c ) <==> pre_gen(a,c )
{
}

predicate post_original(a:array<int>,c:array<int>)
  requires pre_original(a,c)
  reads a, c
{
  (  a[1..] == c[..])
}

predicate post_gen(a:array<int>,c:array<int>)
  requires pre_original(a,c)
  reads a, c
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(a:array<int>,c:array<int>)
  requires pre_original(a,c )
  requires pre_gen(a,c )
  ensures post_original(a,c ) <==> post_gen(a,c )
{
}

