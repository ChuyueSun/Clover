method SlopeSearch(a: array2<int>, key: int) returns (m:int, n:int)
  //TOFILL
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

