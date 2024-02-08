method UpdateElements(a: array<int>)
  requires a.Length >=9
  modifies a
  ensures a[8] == old(a[8])
  ensures a[7]==516
{
  a[4], a[8] := a[4] + 3, a[8] + 1;
  a[7], a[8] := 516, a[8] - 1;
}
