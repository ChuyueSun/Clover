predicate pre_original(str1: seq<char>,str2: seq<char>,prefix: seq<char>){
  true
}

predicate pre_gen(str1: seq<char>,str2: seq<char>,prefix: seq<char>)
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(str1: seq<char>,str2: seq<char>,prefix: seq<char>)
  ensures pre_original(str1,str2,prefix ) <==> pre_gen(str1,str2,prefix )
{
}

predicate post_original(str1: seq<char>,str2: seq<char>,prefix: seq<char>)
  requires pre_original(str1,str2,prefix){
  ( |prefix| <= |str1|) && ( prefix == str1[0..|prefix|]) && ( |prefix| <= |str2|) && ( prefix == str2[0..|prefix|]) && ( |prefix|==|str1| || |prefix|==|str2| || (str1[|prefix|]!=str2[|prefix|]))
}

predicate post_gen(str1: seq<char>,str2: seq<char>,prefix: seq<char>)
  requires pre_original(str1,str2,prefix){
  true // (#POST) && ... (#POST)
}

lemma post_eq(str1: seq<char>,str2: seq<char>,prefix: seq<char>)
  requires pre_original(str1,str2,prefix )
  requires pre_gen(str1,str2,prefix )
  ensures post_original(str1,str2,prefix ) <==> post_gen(str1,str2,prefix )
{
}

