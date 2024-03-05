predicate pre_original(s: array<int>){
  true
}

predicate pre_gen(s: array<int>){
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(s: array<int>)
  ensures pre_original(s) <==> pre_gen(s)
{
}

twostate predicate post_original(s: array<int>)
  requires pre_original(s)
  reads s
{
  forall i :: 0 <= i < s.Length ==> s[i] == 2 * old(s[i])
}

twostate predicate post_gen(s: array<int>)
  requires pre_original(s)
  reads s
{
  true // (#POST) && ... (#POST)
}

twostate lemma post_eq(s: array<int>)
  requires pre_original(s)
  ensures post_original(s) <==> post_gen(s)
{
}

