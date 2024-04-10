method UpdateElements(a: array<int>)

{
  assert a.Length>=9;
  a[4], a[8] := a[4] + 3, a[8] + 1;
  a[7], a[8] := 516, a[8] - 1;
}
