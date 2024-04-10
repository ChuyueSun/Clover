predicate pre_original(s: string, found: bool, c: char)
{
true
}

predicate pre_gen(s: string, found: bool, c: char)
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(s: string, found: bool, c: char)
  ensures pre_original(s, found, c) <==> pre_gen(s, found, c)
{
}

predicate post_original(s: string, found: bool, c: char)
  requires pre_original(s, found, c)
{
true
}

predicate post_gen(s: string, found: bool, c: char)
  requires pre_original(s, found, c)
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(s: string, found: bool, c: char)
  requires pre_original(s, found, c)
  requires pre_gen(s, found, c)
  ensures post_original(s, found, c) <==> post_gen(s, found, c)
{
}