method pqrs(tuvw: array<int>) returns (xyza: int)
  //TOFILL
{
  xyza := tuvw[0];
  var efgh := 1;
  while (efgh < tuvw.Length)
    invariant 0 <= efgh <= tuvw.Length
    invariant forall abcd :: 0 <= abcd < efgh ==> xyza >= tuvw[abcd];
    invariant exists abcd :: 0 <= abcd < efgh && xyza == tuvw[abcd];
    decreases tuvw.Length - efgh
  {
    xyza := if xyza>tuvw[efgh] then  xyza else tuvw[efgh];
    efgh := efgh + 1;
  }
}
