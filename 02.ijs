nums =: ". ;._2 freads '02.txt'
echo +/ ((>./) - (<./))"1 nums

part_two =: 3 : 0"1  NB. Operates on a row of numbers
 NB. pairs where x/y == <. x/y (ignoring x==y)
 a1 =. ((-.=y) *. (<. %/~y) = (%/~y)) # y
 a2 =. ((-.=y) *. (<. %%/~y) = (%%/~y)) # y  NB. same, for inverse
 ab =. , a1 + a2
 div =. %/ (0<ab)#ab
(div<1) { (div, %div)
)

echo +/ part_two nums

exit 0
