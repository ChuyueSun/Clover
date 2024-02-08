method LongestCommonPrefix(str1: seq<char>, str2: seq<char>) returns (prefix: seq<char>)
{
  prefix := [];
  var minLength := if |str1| <|str2| then |str1| else |str2|;

  for idx:= 0 to minLength
  {
    if str1[idx] != str2[idx] {
      return;
    }
    prefix := prefix + [str1[idx]];
  }

}
