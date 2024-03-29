method FindSmallest(s: array<int>) returns (min: int)
  requires s.Length > 0
  ensures forall i :: 0 <= i < s.Length ==> min <= s[i]
  ensures exists i :: 0 <= i < s.Length && min == s[i]
{
  min := s[0];
  for i := 1 to s.Length
    invariant 0 <= i <= s.Length
    invariant forall k :: 0 <= k < i ==> min <= s[k]
    invariant exists k :: 0 <= k < i && min == s[k]
  {
    if s[i] < min
    {
      min := s[i];
    }
  }
}

method FindSmallestTest(){
  var a1:= new int[] [10, 20, 1, 45, 99];
  var out1:=FindSmallest(a1);
  print(out1);print("\n");
              

  var a2:= new int[] [1, 2, 3];
  var out2:=FindSmallest(a2);
  print(out2);print("\n");
              

  var a3:= new int[] [45, 46, 50, 60];
  var out3:=FindSmallest(a3);
  print(out3);print("\n");
              


}

method Main(){
  FindSmallestTest();
}
