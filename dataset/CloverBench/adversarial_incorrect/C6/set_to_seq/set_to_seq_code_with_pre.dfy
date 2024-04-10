method SetToSeq<T>(s: set<T>) returns (xs: seq<T>)
{
  xs := [];
  if s != {} {
    var x :| x in s;
    xs := xs + [x];
  }
}
