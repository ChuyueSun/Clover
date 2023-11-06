method HCJK(mnpm: array<int>, RPWO:int) returns (SCTV: array<int> )
  //TOFILL
{
  SCTV:= new int[mnpm.Length];
  var YFON:=0;
  while YFON<mnpm.Length
    invariant 0<=YFON<=mnpm.Length
    invariant forall PLBV::0<=PLBV<YFON ==> if PLBV+RPWO<mnpm.Length then SCTV[PLBV+RPWO]==mnpm[PLBV] else SCTV[PLBV+RPWO-mnpm.Length]==mnpm[PLBV]
  {
    var FVRJ:= if RPWO+YFON<mnpm.Length then RPWO+YFON else RPWO+YFON-mnpm.Length;
    SCTV[FVRJ]:=mnpm[YFON];
    YFON:=YFON+1;
  }
}
