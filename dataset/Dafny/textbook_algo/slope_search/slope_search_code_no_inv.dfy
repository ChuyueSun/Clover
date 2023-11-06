method SlopeSearch(a: array2<int>, key: int) returns (m:int, n:int)

{
  m,n:=0, a.Length1-1;
  while a[m,n] !=key
  {
    if a[m,n] < key {
      m:=m+1;

    }else{
      n:=n-1;
    }
  }
}

