method SlopeSearch(a: array2<int>, key: int) returns (m:int, n:int)


{
  assert forall i,j,j'::0<=i<a.Length0 && 0<=j<j'<a.Length1 ==> a[i,j]<=a[i,j'];
  assert forall i,i',j::0<=i<i'<a.Length0 && 0<=j<a.Length1 ==> a[i,j]<=a[i',j];
  assert exists i,j :: 0<=i<a.Length0 && 0<=j<a.Length1 && a[i,j]==key;
  assert a.Length0 > 0 && a.Length1 > 0;
  assert a[0, 0] == key;
  m,n:=0, 0;
}

