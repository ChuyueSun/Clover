method foo<T>(s: set<T>) returns (xs: seq<T>)
  ensures |s| == |xs|
{
  xs := [];
  var left: set<T> := s;
  while left != {}
    invariant |multiset(left) + multiset(xs)| == |multiset(s)|
    invariant multiset(left) + multiset(xs) == multiset(s)
  {
    var x :| x in left;
    left := left - {x};
    xs := xs + [x];
  }
}
