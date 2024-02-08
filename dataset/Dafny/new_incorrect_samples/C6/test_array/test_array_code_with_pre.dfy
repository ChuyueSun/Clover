method TestArrayElements(a:array<int>, j: nat, k: nat)

{
  assert j==9;
  assert a.Length>0;
  a[0] := 60;
}
