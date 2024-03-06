predicate pre_original(s: string, p: string, b:bool)
{
  |s| == |p|
}

predicate pre_gen(s: string, p: string, b:bool)
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(s: string, p: string, b:bool)
  ensures pre_original(s,p,b) <==> pre_gen(s,p,b)
{
}

predicate post_original(s: string, p: string, b:bool)
  requires pre_original(s,p,b)
{
  b <==> forall n :: 0 <= n < |s| ==> s[n] == p[n] || p[n] == '*'
}

predicate post_gen(s: string, p: string, b:bool)
  requires pre_original(s,p,b)
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(s: string, p: string, b:bool)
  requires pre_original(s,p,b)
  ensures post_original(s,p,b) <==> post_gen(s,p,b)
{
}

