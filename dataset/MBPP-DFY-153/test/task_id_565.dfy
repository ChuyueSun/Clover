method SplitStringIntoChars(s: string) returns (v: seq<char>)
  ensures |v| == |s|
  ensures forall i :: 0 <= i < |s| ==> v[i] == s[i]
{
  v := [];
  for i := 0 to |s|
    invariant 0 <= i <= |s|
    invariant |v| == i
    invariant forall k :: 0 <= k < i ==> v[k] == s[k]
  {
    v := v + [s[i]];
  }
}





method SplitStringIntoCharsTest(){

  var res1:=SplitStringIntoChars("python");
  print(res1);print("\n");
              

  var res2:=SplitStringIntoChars("Name");
  print(res2);print("\n");
              

  var res3:=SplitStringIntoChars("program");
  print(res3);print("\n");
              


}

method Main(){
  SplitStringIntoCharsTest();
}
