predicate pre_original(nums: array<int>, target: int, i:int, j:int)
  reads nums
{
  (nums.Length > 1)}

predicate pre_gen(nums: array<int>, target: int, i:int, j:int)
  reads nums
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(nums: array<int>, target: int, i:int, j:int)
  ensures pre_original(nums,target,i,j) <==> pre_gen(nums,target,i ,j)
{
}

ghost predicate post_original(nums: array<int>, target: int, i:int, j:int)
  requires pre_original(nums,target,i, j)
  reads nums
{
  true
}
ghost predicate post_gen(nums: array<int>, target: int, i:int, j:int)
  requires pre_original(nums,target,i, j)
  reads nums
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(nums: array<int>, target: int, i:int, j:int)
  requires pre_original(nums,target,i,j)
  requires pre_gen(nums,target,i,j)
  ensures post_original(nums,target,i,j) <==> post_gen(nums,target,i,j)
{
}

