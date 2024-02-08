predicate pre_original(a:array<int>,b:int,c:array<int>)
  reads a, c
{
  a.Length>0
}

predicate pre_gen(a:array<int>,b:int,c:array<int>)
  reads a, c
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(a:array<int>,b:int,c:array<int>)
  ensures pre_original(a,b,c ) <==> pre_gen(a,b,c )
{
}

predicate post_original(a:array<int>,b:int,c:array<int>)
  requires pre_original(a,b,c)
  reads a, c
{
  c.Length == a.Length +1
}

predicate post_gen(a:array<int>,b:int,c:array<int>)
  requires pre_original(a,b,c)
  reads a, c
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(a:array<int>,b:int,c:array<int>)
  requires pre_original(a,b,c )
  requires pre_gen(a,b,c )
  ensures post_original(a,b,c ) <==> post_gen(a,b,c )
{
}

