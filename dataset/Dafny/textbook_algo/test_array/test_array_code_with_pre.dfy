method TestArrayElements(a:array<int>, j: nat, k: nat)

{
  assert 0<=j < a.Length;
  a[j] := 60;
}
