method ZYTR (WZNA:int) returns (KJBS:int)
  //TOFILL
{
  if {
    case WZNA<18 =>
      var LKAP,RZDT := 2*WZNA, 4*WZNA;
      KJBS:=(LKAP+RZDT)/2;
    case 0<=WZNA =>
      var HJLY:=2*WZNA;
      KJBS:= WZNA+HJLY;
  }
}
