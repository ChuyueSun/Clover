predicate pre_original(a: array<int>, result: int)
reads a
{
true
}

predicate pre_gen(a: array<int>, result: int)
reads a
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(a: array<int>, result: int)
  ensures pre_original(a, result) <==> pre_gen(a, result)
{
}

predicate post_original(a: array<int>, result: int)
  requires pre_original(a, result)
  reads a
{
true
}

predicate post_gen(a: array<int>, result: int)
  requires pre_original(a, result)
  reads a
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(a: array<int>, result: int)
  requires pre_original(a, result)
  requires pre_gen(a, result)
  ensures post_original(a, result) <==> post_gen(a, result)
{
}

function sumTo( a:array<int>, n:int ) : int
  requires 0 <= n && n <= a.Length
  decreases n
  reads a
{
  if (n == 0) then 0 else sumTo(a, n-1) + a[n-1]
}