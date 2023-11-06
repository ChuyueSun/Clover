method UpdateElements(a: array<int>)
  requires a.Length >=9
  modifies a
  ensures old(a[4]) +3 == a[4]
  ensures a[8] == old(a[8])
  ensures a[7]==516
{
  //TOFILL
}
