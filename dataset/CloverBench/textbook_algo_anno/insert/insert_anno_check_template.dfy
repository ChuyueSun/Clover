predicate pre_original(line:array<char>, l:int, nl:array<char>, p:int, at:int){
  (0 <= l+p <= line.Length )&&
  (0 <= p <= nl.Length )&&
  (0 <= at <= l )
}

predicate pre_gen(line:array<char>, l:int, nl:array<char>, p:int, at:int){
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(line:array<char>, l:int, nl:array<char>, p:int, at:int)
  ensures pre_original(line,l,nl,p,at ) <==> pre_gen(line,l,nl,p,at )
{
}

twostate predicate post_original(line:array<char>, l:int, nl:array<char>, p:int, at:int)
  requires pre_original(line,l,nl,p,at)
  reads line, nl
{
  (forall i :: (0<=i<p) ==> line[at+i] == nl[i] )&&
  (forall i :: (0<=i<at) ==> line[i] == old(line[i])) &&
  (forall i :: (at+p<=i<l+p) ==> line[i] == old(line[i-p]))
}

twostate predicate post_gen(line:array<char>, l:int, nl:array<char>, p:int, at:int)
  requires pre_original(line,l,nl,p,at)
  reads line, nl
{
  true // (#POST) && ... (#POST)
}

twostate lemma post_eq(line:array<char>, l:int, nl:array<char>, p:int, at:int)
  requires pre_original(line,l,nl,p,at )
  requires pre_gen(line,l,nl,p,at )
  ensures post_original(line,l,nl,p,at ) <==> post_gen(line,l,nl,p,at )
{
}

