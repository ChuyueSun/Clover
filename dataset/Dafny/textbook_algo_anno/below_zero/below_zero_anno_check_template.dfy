predicate pre_original(operations: seq<int>, s:array<int>, result:bool)
{
  true
}

predicate pre_gen(operations: seq<int>, s:array<int>, result:bool)
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(operations: seq<int>,  s:array<int>, result:bool)
  ensures pre_original(operations, s, result) <==> pre_gen(operations, s, result )
{
}

predicate post_original(operations: seq<int>, s:array<int>, result:bool)
  requires pre_original(operations, s, result)
  reads s
{
  (s.Length == |operations| + 1) &&
  (s[0]==0) &&
  (forall i :: 0 <= i < s.Length-1 ==> s[i+1]==s[i]+operations[i]) &&
  (result == true ==> (exists i :: 1 <= i <= |operations| && s[i] < 0)) &&
  (result == false ==> forall i :: 0 <= i < s.Length ==> s[i] >= 0)
}

predicate post_gen(operations: seq<int>, s:array<int>, result:bool)
  requires pre_original(operations, s, result)
  reads s
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(operations: seq<int>, s:array<int>, result:bool)
  requires pre_original(operations, s, result)
  requires pre_gen(operations, s, result)
  ensures post_original(operations, s, result) <==> post_gen(operations, s, result)
{
}

