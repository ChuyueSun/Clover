  method allDigits(s: string) returns (result: bool)
{
  result:=true ;
  for i := 0 to |s|
  {
    if ! (s[i] in "0123456789"){
      return false;
    }
  }
}
