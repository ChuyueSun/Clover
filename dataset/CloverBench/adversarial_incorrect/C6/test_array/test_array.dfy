method TestArrayElements(a:array<int>, j: nat, k: nat)
  requires j == 0
  requires a.Length > 0
  modifies a
  ensures a[j] == 60
{
  a[0] := 60;
}
