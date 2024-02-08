method SlopeSearch(a: array2<int>, key: int) returns (m:int, n:int)
{
  m,n:=0, 0;

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
