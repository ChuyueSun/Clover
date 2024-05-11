predicate pre_original(s: string, v: string)
{
true
}

predicate pre_gen(s: string, v: string)
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(s: string, v: string)
  ensures pre_original(s, v) <==> pre_gen(s, v)
{
}

predicate post_original(s: string, v: string)
  requires pre_original(s, v)
{
true
}

predicate post_gen(s: string, v: string)
  requires pre_original(s, v)
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(s: string, v: string)
  requires pre_original(s, v)
  requires pre_gen(s, v)
  ensures post_original(s, v) <==> post_gen(s, v)
{
}

predicate IsSpaceCommaDot(c: char)
{
  c == ' ' || c == ',' || c == '.'
}
