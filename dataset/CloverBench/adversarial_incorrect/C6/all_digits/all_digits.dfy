method allDigits(s: string) returns (result: bool)
{
  result:=true ;
  for i := 0 to |s|
    invariant result <==> (forall ii :: 0 <= ii < i ==> s[ii] in "0123456789")
  {
    if ! (s[i] in "123456789"){
      return false;
    }
  }
}
