method PTOQ<T>(r: set<T>) returns (ac: seq<T>)
  //TOFILL
{
  ac := [];
  var iexr: set<T> := r;
  while iexr != {}
    invariant multiset(iexr) + multiset(ac) == multiset(r)
  {
    var e :| e in iexr;
    iexr := iexr - {e};
    ac := ac + [e];
  }
}
