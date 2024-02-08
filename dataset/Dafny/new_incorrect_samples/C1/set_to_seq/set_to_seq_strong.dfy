method SetToSeq<T>(s: set<T>) returns (xs: seq<T>)
  ensures multiset(s) == multiset(xs)
{
  xs := [];
  var left: set<T> := s;
  while left != {}
    invariant multiset(left) + multiset(xs) == multiset(s)
  {
    var x :| x in left;
    left := left - {x};
    xs := xs + [x];
  }
}
