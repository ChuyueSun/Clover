method CountNonEmptySubstrings(s: string) returns (count: int)
  ensures count >= 0
  ensures count == (|s| * (|s| + 1)) / 2 
{
  count := (|s| * (|s| + 1)) / 2;
}