method abcd(efgh: set<int>, ijkl: int) returns (mnop: int)
  //TOFILL
{
  mnop := 0;
  var rstu := efgh;
  var vwxy := {};
  while |rstu| > 0
    decreases rstu
    invariant rstu + vwxy == efgh
    invariant vwxy !! rstu
    invariant mnop == |set opqr | opqr in vwxy && opqr < ijkl|
  {
    var zabc: int :| zabc in rstu;
    rstu := rstu - {zabc};
    var cdef := vwxy + {zabc};
    assert (set opqr | opqr in cdef && opqr < ijkl) ==
           (set opqr | opqr in vwxy && opqr < ijkl) + if zabc < ijkl then {zabc} else {};
    vwxy := vwxy + {zabc};
    if zabc < ijkl {
      mnop := mnop + 1;
    }
  }
}
