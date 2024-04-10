method CanyonSearch(a: array<int>, b: array<int>) returns (d:nat)
  requires a.Length !=0 && b.Length!=0
  requires forall i,j :: 0<=i<j<a.Length ==> a[i]<=a[j]
  requires forall i,j :: 0<=i<j<b.Length ==> b[i]<=b[j]
  ensures exists i,j:: 0<=i<a.Length && 0<=j<b.Length && d==if a[i] < b[j] then (b[j]-a[i]) else (a[i]-b[j])
  ensures forall i,j:: 0<=i<a.Length && 0<=j<b.Length ==> d<=if a[i] < b[j] then (b[j]-a[i]) else (a[i]-b[j])
{
  var m,n:=0,0;
  d:=if a[0] < b[0] then (b[0]-a[0]) else (a[0]-b[0]);
  while m<a.Length && n<b.Length
    invariant 0<=m<=a.Length && 0<=n<=b.Length
    decreases a.Length -m+b.Length-n
    invariant exists i,j:: 0<=i<a.Length && 0<=j<b.Length && d==if a[i] < b[j] then (b[j]-a[i]) else (a[i]-b[j])
    invariant forall i,j:: 0<=i<a.Length && 0<=j<b.Length ==> d<=(if a[i] < b[j] then (b[j]-a[i]) else (a[i]-b[j]))|| (m<=i&&n<=j)
  {
    var t := if a[m] < b[n] then (b[n]-a[m]) else (a[m]-b[n]);
    d:=if t<d then t else d;
    if
    case a[m]<=b[n] =>
      m:=m+1;
    case b[n]<=a[m] =>
      n:=n+1;

  }
}


method TestCanyonSearch()
{
  var a := new int[5] [1, 2, 3, 4, 5];
  var b := new int[3] [1, 3, 5];
  var d := CanyonSearch(a, b);
  print("Test 1: CanyonSearch([1, 2, 3, 4, 5], [1, 3, 5]) = ", d, "\n");


  a := new int[5] [1, 3, 5, 7, 9];
  b := new int[3] [2, 4, 6];
  d := CanyonSearch(a, b);
  print("Test 2: CanyonSearch([1, 3, 5, 7, 9], [2, 4, 6]) = ", d, "\n");


  a := new int[5] [2, 4, 6, 8, 10];
  b := new int[3] [1, 3, 5];
  d := CanyonSearch(a, b);
  print("Test 3: CanyonSearch([2, 4, 6, 8, 10], [1, 3, 5]) = ", d, "\n");

  a := new int[5] [1, 1,1,1,1];
  b := new int[3] [1, 1, 1];
  d := CanyonSearch(a, b);
  print("Test 4: CanyonSearch([1, 1, 1, 1, 1], [1, 1, 1]) = ", d, "\n");


  a := new int[5] [5, 10, 15, 20, 25];
  b := new int[3] [5, 10, 15];
  d := CanyonSearch(a, b);
  print("Test 5: CanyonSearch([5, 10, 15, 20, 25], [5, 10, 15]) = ", d, "\n");

}

method Main()
{
  TestCanyonSearch();
}
