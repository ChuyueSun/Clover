method AppendArrayToSeq(s: seq<int>, a: array<int>) returns (r: seq<int>)
  requires a != null
  ensures |r| == |s| + a.Length
  ensures forall i :: 0 <= i < |s| ==> r[i] == s[i]
  ensures forall i :: 0 <= i < a.Length ==> r[|s| + i] == a[i]
{
  r := s;
  for i := 0 to a.Length
    invariant 0 <= i <= a.Length
    invariant |r| == |s| + i
    invariant forall j :: 0 <= j < |s| ==> r[j] == s[j]
    invariant forall j :: 0 <= j < i ==> r[|s| + j] == a[j]
  {
    r := r + [a[i]];
  }
}

method AppendArrayToSeqTest(){
  var s1: seq<int> := [9, 10];
  var a1:= new int[] [5, 6, 7];
  var res1:=AppendArrayToSeq(s1,a1);
  print(res1);print("\n");
              

  var s2: seq<int> := [10, 11];
  var a2:= new int[] [6, 7, 8];
  var res2:=AppendArrayToSeq(s2,a2);
  print(res2);print("\n");
              

  var s3: seq<int> := [11, 12];
  var a3:= new int[] [7, 8, 9];
  var res3:=AppendArrayToSeq(s3,a3);
  print(res3);print("\n");
              

}

method Main(){
  AppendArrayToSeqTest();
}