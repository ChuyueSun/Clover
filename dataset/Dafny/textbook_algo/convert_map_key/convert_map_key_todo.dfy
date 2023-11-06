method convert_map_key(inputs: map<nat, bool>, f: nat->nat) returns(r:map<nat, bool>)
  //TOFILL
{
  r:= map k | k in inputs :: f(k) := inputs[k];

}
