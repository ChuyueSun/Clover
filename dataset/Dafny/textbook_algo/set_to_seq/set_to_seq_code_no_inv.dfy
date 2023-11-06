method SetToSeq<T>(s: set<T>) returns (xs: seq<T>)
{
  xs := [];
  var left: set<T> := s;
  while left != {}
  {
    var x :| x in left;
    left := left - {x};
    xs := xs + [x];
  }

}
