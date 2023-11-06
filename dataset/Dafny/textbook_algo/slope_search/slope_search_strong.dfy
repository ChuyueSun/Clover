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

