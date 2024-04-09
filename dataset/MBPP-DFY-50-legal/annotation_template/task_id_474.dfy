predicate pre_original(s: string, oldChar: char, newChar: char, v: string)
{
true
}

predicate pre_gen(s: string, oldChar: char, newChar: char, v: string)
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(s: string, oldChar: char, newChar: char, v: string)
  ensures pre_original(s, oldChar, newChar, v) <==> pre_gen(s, oldChar, newChar, v)
{
}

predicate post_original(s: string, oldChar: char, newChar: char, v: string)
  requires pre_original(s, oldChar, newChar, v)
{
true
}

predicate post_gen(s: string, oldChar: char, newChar: char, v: string)
  requires pre_original(s, oldChar, newChar, v)
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(s: string, oldChar: char, newChar: char, v: string)
  requires pre_original(s, oldChar, newChar, v)
  requires pre_gen(s, oldChar, newChar, v)
  ensures post_original(s, oldChar, newChar, v) <==> post_gen(s, oldChar, newChar, v)
{
}