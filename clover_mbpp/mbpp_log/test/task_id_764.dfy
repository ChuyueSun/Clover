predicate IsDigit(c: char)
{
  48 <= c as int <= 57
}


method CountDigits(s: string) returns (count: int)
  ensures count >= 0
  ensures count == | set i: int | 0 <= i < |s| && IsDigit(s[i])|
{
  var digits := set i: int | 0 <= i < |s| && IsDigit(s[i]);
  count := |digits|;
}




method CountDigitsTest(){
  var out1:=CountDigits("program2bedone");
  print(out1);print("\n");
              

  var out2:=CountDigits("3wonders");
  print(out2);print("\n");
              

  var out3:=CountDigits("3wond-1ers2");
  print(out3);print("\n");
              


}

method Main(){
  CountDigitsTest();
}
