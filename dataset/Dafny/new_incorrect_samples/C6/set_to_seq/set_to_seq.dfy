method SetToSeq<T>(s: set<T>) returns (xs: seq<T>)
  ensures multiset(s) >= multiset(xs)
{
  xs := [];
  if s != {} {
    var x :| x in s;
    xs := xs + [x];
  }
}
