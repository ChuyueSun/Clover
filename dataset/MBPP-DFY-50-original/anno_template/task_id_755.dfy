predicate pre_original(s: array<int>, secondSmallest: int)
reads s
{
true
}

predicate pre_gen(s: array<int>, secondSmallest: int)
reads s
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(s: array<int>, secondSmallest: int)
  ensures pre_original(s, secondSmallest) <==> pre_gen(s, secondSmallest)
{
}

predicate post_original(s: array<int>, secondSmallest: int)
  requires pre_original(s, secondSmallest)
  reads s
{
true
}

predicate post_gen(s: array<int>, secondSmallest: int)
  requires pre_original(s, secondSmallest)
  reads s
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(s: array<int>, secondSmallest: int)
  requires pre_original(s, secondSmallest)
  requires pre_gen(s, secondSmallest)
  ensures post_original(s, secondSmallest) <==> post_gen(s, secondSmallest)
{
}

function MinPair(s: seq<int>) : (r: int)
  requires |s| == 2
  ensures s[0] <= s[1] <==> r == s[0]
  ensures s[0] > s[1] ==> r == s[1]
{
  if s[0] <= s[1] then s[0] else s[1]
}


function min(s: seq<int>) : (r: int)
  requires |s| >= 2
  ensures forall i :: 0 <= i < |s| ==> r <= s[i]
{
  if |s| == 2 then MinPair(s)
  else MinPair([s[0], min(s[1..])])
}

