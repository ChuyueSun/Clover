method IsPalindrome(x: seq<char>) returns (result: bool)
  ensures result <==> (forall i :: 0 <= i < |x| / 2 ==> x[i] == x[i + |x| - |x| / 2])
{
  if |x|==0 {
    return true;
  }
  var i := 0;
  result := true;
  while (i < |x| / 2)
    invariant 0<=i<= |x| / 2
    invariant (forall k :: 0 <= k < i ==> x[k] == x[k + |x| - |x| / 2])
  {
    if x[i] != x[i + |x| - |x| / 2] {
      result := false;
      return;
    }
    i := i + 1;
  }
}
