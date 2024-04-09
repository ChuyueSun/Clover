predicate pre_original(N:int, s:int){
  N >= 0
}

predicate pre_gen(N:int, s:int){
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(N:int, s:int)
  ensures pre_original(N, s) <==> pre_gen(N, s)
{
}

predicate post_original(N:int, s:int)
  requires pre_original(N, s){
  s == N * (N + 1) / 2
}

predicate post_gen(N:int, s:int)
  requires pre_original(N, s){
  true // (#POST) && ... (#POST)
}

lemma post_eq(N:int, s:int)
  requires pre_original(N, s)
  ensures post_original(N, s) <==> post_gen(N, s)
{
}

