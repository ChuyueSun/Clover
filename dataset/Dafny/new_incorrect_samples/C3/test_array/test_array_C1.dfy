method TestArrayElements(a:array<int>, j: nat, k: nat)
  requires 0<=j < a.Length
  modifies a
{
  a[j] := 60;
}
