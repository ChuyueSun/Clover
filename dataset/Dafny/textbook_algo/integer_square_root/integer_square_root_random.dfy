method AXYZ(UINO:nat) returns (RDPR:nat)
  //TOFILL
{
  RDPR:=0;
  while (RDPR+1)*(RDPR+1)<=UINO
    invariant RDPR*RDPR<=UINO
  {
    RDPR:=RDPR+1;
  }
}
