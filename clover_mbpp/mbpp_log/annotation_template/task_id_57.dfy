predicate pre_original()
{
true
}

predicate pre_gen()
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq()
  ensures pre_original() <==> pre_gen()
{
}

predicate post_original()
  requires pre_original()
{
true
}

predicate post_gen()
  requires pre_original()
{
  true // (#POST) && ... (#POST)
}

lemma post_eq()
  requires pre_original()
  requires pre_gen()
  ensures post_original() <==> post_gen()
{
}