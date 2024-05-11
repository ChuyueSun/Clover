predicate pre_original(sub: seq<int>, main: seq<int>, result: bool)
{
true
}

predicate pre_gen(sub: seq<int>, main: seq<int>, result: bool)
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(sub: seq<int>, main: seq<int>, result: bool)
  ensures pre_original(sub, main, result) <==> pre_gen(sub, main, result)
{
}

predicate post_original(sub: seq<int>, main: seq<int>, result: bool)
  requires pre_original(sub, main, result)
{
true
}

predicate post_gen(sub: seq<int>, main: seq<int>, result: bool)
  requires pre_original(sub, main, result)
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(sub: seq<int>, main: seq<int>, result: bool)
  requires pre_original(sub, main, result)
  requires pre_gen(sub, main, result)
  ensures post_original(sub, main, result) <==> post_gen(sub, main, result)
{
}