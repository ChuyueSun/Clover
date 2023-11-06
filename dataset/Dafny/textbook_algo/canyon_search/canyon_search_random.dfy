method WXYZ(ABCD: array<int>, EFGH: array<int>) returns (IJKL:nat)
  //TOFILL
{
  var BCDE,FGHI:=0,0;
  IJKL:=if ABCD[0] < EFGH[0] then (EFGH[0]-ABCD[0]) else (ABCD[0]-EFGH[0]);
  while BCDE<ABCD.Length && FGHI<EFGH.Length
    invariant 0<=BCDE<=ABCD.Length && 0<=FGHI<=EFGH.Length
    decreases ABCD.Length -BCDE+EFGH.Length-FGHI
    invariant exists JKLM,NOOP:: 0<=JKLM<ABCD.Length && 0<=NOOP<EFGH.Length && IJKL==if ABCD[JKLM] < EFGH[NOOP] then (EFGH[NOOP]-ABCD[JKLM]) else (ABCD[JKLM]-EFGH[NOOP])
    invariant forall JKLM,NOOP:: 0<=JKLM<ABCD.Length && 0<=NOOP<EFGH.Length ==> IJKL<=(if ABCD[JKLM] < EFGH[NOOP] then (EFGH[NOOP]-ABCD[JKLM]) else (ABCD[JKLM]-EFGH[NOOP]))|| (BCDE<=JKLM&&FGHI<=NOOP)
  {
    var PQRS := if ABCD[BCDE] < EFGH[FGHI] then (EFGH[FGHI]-ABCD[BCDE]) else (ABCD[BCDE]-EFGH[FGHI]);
    IJKL:=if PQRS<IJKL then PQRS else IJKL;
    if
    case ABCD[BCDE]<=EFGH[FGHI] =>
      BCDE:=BCDE+1;
    case EFGH[FGHI]<=ABCD[BCDE] =>
      FGHI:=FGHI+1;
  }
}
