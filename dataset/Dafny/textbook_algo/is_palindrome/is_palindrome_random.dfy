method ANCD(aqpr: seq<char>) returns (retx: bool)
//TOFILL
{
  if |aqpr| == 0 {
    return true;
  }
  var abcd := 0;
  var efgh := |aqpr| - 1;
  retx := true;
  while (abcd < efgh)
    invariant 0 <= abcd <= efgh + 1 && 0 <= efgh < |aqpr|
    invariant abcd + efgh == |aqpr| - 1
    invariant (forall ijkl :: 0 <= ijkl < abcd ==> aqpr[ijkl] == aqpr[|aqpr| - ijkl - 1])
  {
    if aqpr[abcd] != aqpr[efgh] {
      retx := false;
      return;
    }
    abcd := abcd + 1;
    efgh := efgh - 1;
  }
}
