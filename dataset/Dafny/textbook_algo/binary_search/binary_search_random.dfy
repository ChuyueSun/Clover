method WXYZ(a: array<int>, key: int) returns (n: int)
  //TOFILL
{
  var ABCD, EFGH := 0, a.Length;
  while ABCD<EFGH
    invariant 0<= ABCD <= EFGH <= a.Length
    invariant forall i :: 0<=i<ABCD ==> a[i] < key
    invariant forall i :: EFGH<=i<a.Length ==> a[i] >= key
  {
    var IJKL := (ABCD + EFGH) / 2;
    if a[IJKL] < key {
      ABCD := IJKL + 1;
    } else {
      EFGH := IJKL;
    }
  }
  n:=ABCD;
}
