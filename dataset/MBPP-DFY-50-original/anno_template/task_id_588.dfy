predicate pre_original(a: array<int>, diff: int)
reads a
{
true
}

predicate pre_gen(a: array<int>, diff: int)
reads a
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(a: array<int>, diff: int)
  ensures pre_original(a, diff) <==> pre_gen(a, diff)
{
}

predicate post_original(a: array<int>, diff: int)
  requires pre_original(a, diff)
  reads a
{
true
}

predicate post_gen(a: array<int>, diff: int)
  requires pre_original(a, diff)
  reads a
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(a: array<int>, diff: int)
  requires pre_original(a, diff)
  requires pre_gen(a, diff)
  ensures post_original(a, diff) <==> post_gen(a, diff)
{
}

function Min(a: seq<int>) : int
  requires |a| > 0
{
  if |a| == 1 then a[0]
  else
    var minPrefix := Min(a[..|a|-1]);
    if a[|a|-1] <= minPrefix then a[|a|-1] else Min(a[..|a|-1])
}

function Max(a: seq<int>) : int
  requires |a| > 0
{
  if |a| == 1 then a[0]
  else
    var maxPrefix := Max(a[..|a|-1]);
    if a[|a|-1] >= maxPrefix then a[|a|-1] else Max(a[..|a|-1])
}

