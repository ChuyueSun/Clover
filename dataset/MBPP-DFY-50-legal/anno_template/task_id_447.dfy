predicate pre_original(a: array<int>, cubed: array<int>)
reads a, cubed
{
true
}

predicate pre_gen(a: array<int>, cubed: array<int>)
reads a, cubed
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(a: array<int>, cubed: array<int>)
  ensures pre_original(a, cubed) <==> pre_gen(a, cubed)
{
}

predicate post_original(a: array<int>, cubed: array<int>)
  requires pre_original(a, cubed)
  reads a, cubed
{
true
}

predicate post_gen(a: array<int>, cubed: array<int>)
  requires pre_original(a, cubed)
  reads a, cubed
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(a: array<int>, cubed: array<int>)
  requires pre_original(a, cubed)
  requires pre_gen(a, cubed)
  ensures post_original(a, cubed) <==> post_gen(a, cubed)
{
}