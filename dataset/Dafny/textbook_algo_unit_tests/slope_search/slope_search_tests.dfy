method SlopeSearch(a: array2<int>, key: int) returns (m:int, n:int)
  requires forall i,j,j'::0<=i<a.Length0 && 0<=j<j'<a.Length1 ==> a[i,j]<=a[i,j']
  requires forall i,i',j::0<=i<i'<a.Length0 && 0<=j<a.Length1 ==> a[i,j]<=a[i',j]
  requires exists i,j :: 0<=i<a.Length0 && 0<=j<a.Length1 && a[i,j]==key
  ensures 0<=m<a.Length0 && 0<=n<a.Length1
  ensures a[m,n]==key
{
  m,n:=0, a.Length1-1;
  while a[m,n] !=key
    invariant 0<=m<a.Length0 && 0<=n<a.Length1
    invariant exists i,j :: m<=i<a.Length0 && 0<=j<=n && a[i,j]==key
    decreases a.Length0-m+n
  {
    if a[m,n] < key {
      m:=m+1;

    }else{
      n:=n-1;
    }
  }
}


method TestSlopeSearch() {
  var a := new int[3,3]((i, j) => i+j);
  assert a[0,0]==0;
  var m, n := SlopeSearch(a, 0);
  print("Test 1: SlopeSearch(...) = ", m,n, "\n");

  assert a[1,0]==1;
  m, n := SlopeSearch(a, 1);
  print("Test 2: SlopeSearch(...) = ", m,n, "\n");
  assert a[2,0]==2;
  m, n := SlopeSearch(a, 2);
  print("Test 4: SlopeSearch(...) = ", m,n, "\n");

  assert a[1,1]==2;
  m, n := SlopeSearch(a, 2);
  print("Test 4: SlopeSearch(...) = ", m,n, "\n");

  assert a[2,2]==4;
  m, n := SlopeSearch(a, 4);
  print("Test 5: SlopeSearch(...) = ", m,n, "\n");

}

method Main() {
  TestSlopeSearch();
}
