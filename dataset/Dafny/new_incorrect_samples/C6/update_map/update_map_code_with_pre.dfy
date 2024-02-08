method update_map<K(!new), V>(m1: map<K, V>, m2: map<K, V>) returns (r: map<K, V>)

{
  r:= map k | k in (m1.Keys + m2.Keys) :: if k in m1 then m1[k] else m2[k];
}
