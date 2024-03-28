method FindWords(s: string) returns (v: seq<string>)
    ensures forall w :: w in v ==> |w| >= 4 && w in s
    ensures forall w :: w in s && |w| >= 4 ==> w in v
{
    var words := s.Split(' ');
    v := [];

    for i := 0 to |words|
        invariant 0 <= i <= |words|
        invariant forall w :: w in v ==> |w| >= 4 && w in words
        invariant forall w :: w in words[0..i] && |w| >= 4 ==> w in v
    {
        if |words[i]| >= 4 {
            v := v + [words[i]];
        }
    }
}