predicate pre_original(N:nat,r:nat){
  true
}

predicate pre_gen(N:nat,r:nat){
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(N:nat,r:nat)
  ensures pre_original(N,r ) <==> pre_gen(N,r )
{
}

predicate post_original(N:nat,r:nat)
  requires pre_original(N,r){
  ( r*r <= N < (r+1)*(r+1))
}

predicate post_gen(N:nat,r:nat)
  requires pre_original(N,r){
  true // (#POST) && ... (#POST)
}

lemma post_eq(N:nat,r:nat)
  requires pre_original(N,r )
  requires pre_gen(N,r )
  ensures post_original(N,r ) <==> post_gen(N,r )
{
}

