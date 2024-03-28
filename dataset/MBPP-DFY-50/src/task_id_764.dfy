method CountDigits(s: string) returns (count: int)
  ensures count >= 0
  ensures count == | set i: int | 0 <= i < |s| && (48 <= s[i] as int <= 57)|
{
  var digits := set i: int | 0 <= i < |s| && (48 <= s[i] as int <= 57);
  count := |digits|;
}

