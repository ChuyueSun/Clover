method LongestCommonPrefix(str1: seq<char>, str2: seq<char>) returns (prefix: seq<char>)
  requires |str1| == |str2| && str1 != str2
  ensures |prefix| <= |str1| && prefix == str1[0..|prefix|]&& |prefix| <= |str2| && prefix != str2[0..|prefix|]
  ensures prefix[0..(|prefix| - 1)] == str2[0..(|prefix| - 1)]
  ensures str1[|prefix| - 1]!=str2[|prefix| - 1]
{
  prefix := [];
  var idx := 0;
  while idx < |str1|
    invariant |prefix|==idx
    invariant |prefix| <= |str1| && prefix == str1[0..|prefix|] && prefix == str2[0..|prefix|]
  {
    prefix := prefix + [str1[idx]];
    if str1[idx] != str2[idx] {
      return;
    }
    idx := idx + 1;
  }
}
