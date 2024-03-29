predicate IsSpaceCommaDot(c: char)
{
  c == ' ' || c == ',' || c == '.'
}

method ReplaceWithColon(s: string) returns (v: string)
  ensures |v| == |s|
  ensures forall i :: 0 <= i < |s| ==> (IsSpaceCommaDot(s[i]) ==> v[i] == ':') && (!IsSpaceCommaDot(s[i]) ==> v[i] == s[i])
{
  var s' : string := [];
  for i := 0 to |s|
    invariant 0 <= i <= |s|
    invariant |s'| == i
    invariant forall k :: 0 <= k < i ==> (IsSpaceCommaDot(s[k]) ==> s'[k] == ':') && (!IsSpaceCommaDot(s[k]) ==> s'[k] == s[k])
  {
    if IsSpaceCommaDot(s[i])
    {
      s' := s' + [':'];
    }
    else
    {
      s' := s' + [s[i]];
    }
  }
  return s';
}


method ReplaceWithColonTest(){
  var out1:=ReplaceWithColon("Python language, Programming language.");
  print(out1);print("\n");
              

  var out2:=ReplaceWithColon("a b c,d e f");
  print(out2);print("\n");
              

  var out3:=ReplaceWithColon("ram reshma,ram rahim");
  print(out3);print("\n");
              

}

method Main(){
  ReplaceWithColonTest();
}

