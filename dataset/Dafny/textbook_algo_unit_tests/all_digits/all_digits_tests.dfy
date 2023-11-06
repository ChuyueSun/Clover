  method allDigits(s: string) returns (result: bool)
  ensures  result <==> (forall i :: 0 <= i < |s| ==> s[i] in "0123456789")
{
  result:=true ;
  for i := 0 to |s|
    invariant result <==> (forall ii :: 0 <= ii < i ==> s[ii] in "0123456789")
  {
    if ! (s[i] in "0123456789"){
      return false;
    }
  }
}

method TestAbs()
{
  var s:= "1234567890";

  var test1 := allDigits(s);
  print( test1, "\n"); // Expected output: 5
  s:= "123a456";
  var test2 := allDigits(s);
  print(test2, "\n"); // Expected output: 7
  s:="";
  var test3 := allDigits(s);
  print(test3, "\n"); // Expected output: 0
  s:="         ";
  var test4 := allDigits(s);
  print( test4, "\n"); // Expected output: 1
  s:="@#$$%^&";
  var test5 := allDigits(s);
  print( test5, "\n"); // Expected output: 9
}


method Main()
{
  TestAbs();
}
