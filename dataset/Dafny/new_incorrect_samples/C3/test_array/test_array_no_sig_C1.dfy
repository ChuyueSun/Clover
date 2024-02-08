method foo(a:array<int>, j: nat, k: nat)
  requires 0<=j < a.Length
  modifies a
  ensures a[j] >= 60
{
  a[j] := 60;
}
