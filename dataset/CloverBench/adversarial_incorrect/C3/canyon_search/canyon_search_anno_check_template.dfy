predicate pre_original(a: array<int>,b: array<int>,d:nat)
  reads a, b
{
  ( a.Length !=0) && ( b.Length!=0) && ( forall i,j :: 0<=i<j<a.Length ==> a[i]<=a[j]) && ( forall i,j :: 0<=i<j<b.Length ==> b[i]<=b[j])
}

predicate pre_gen(a: array<int>,b: array<int>,d:nat)
  reads a, b
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(a: array<int>,b: array<int>,d:nat)
  ensures pre_original(a,b,d ) <==> pre_gen(a,b,d )
{
}

predicate post_original(a: array<int>,b: array<int>,d:nat)
  requires pre_original(a,b,d)
  reads a, b
{
  ( forall i,j:: 0<=i<a.Length && 0<=j<b.Length ==> d<=if a[i] < b[j] then (b[j]-a[i]) else (a[i]-b[j]))
}

predicate post_gen(a: array<int>,b: array<int>,d:nat)
  requires pre_original(a,b,d)
  reads a, b
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(a: array<int>,b: array<int>,d:nat)
  requires pre_original(a,b,d )
  requires pre_gen(a,b,d )
  ensures post_original(a,b,d ) <==> post_gen(a,b,d )
{
}

