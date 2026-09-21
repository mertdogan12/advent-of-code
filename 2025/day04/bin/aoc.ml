module IntPair = struct
  type t = int * int
  let compare = compare
end

module XYSet = Set.Make(IntPair);;

let explode s =
  let rec expl i l =
    if i < 0 then l else
    expl (i - 1) (s.[i] :: l) in
  expl (String.length s - 1) []
;;

let implode l =
  let rec imp s l =
    match l with
    | x :: y -> imp (s ^ string_of_int x) y
    | [] -> s in
  imp "" l
;;

let parse_line line set y =
  let rec pl l x s =
    match l with
    | i :: j ->
      let s = if i == '@' then XYSet.add (x, y) s else s in
      pl j (x + 1) s
    | [] -> s in
  pl (explode line) 0 set
;;

let parse ic =
  let rec p ic y set =
    match input_line ic with
    | line ->
      let set =  parse_line line set y in
      p ic (y + 1) set
    | exception End_of_file -> set
  in p ic 0 XYSet.empty
;;

let is_grabbeble set xy =
  let neighbor_count set xy =
    let count = if XYSet.mem (fst xy - 1, snd xy - 1) set then 1 else 0 in
    let count = if XYSet.mem (fst xy, snd xy - 1) set then (count + 1) else count in
    let count = if XYSet.mem (fst xy + 1, snd xy - 1) set then (count + 1) else count in
    let count = if XYSet.mem (fst xy - 1, snd xy) set then (count + 1) else count in
    let count = if XYSet.mem (fst xy + 1, snd xy) set then (count + 1) else count in
    let count = if XYSet.mem (fst xy - 1, snd xy + 1) set then (count + 1) else count in
    let count = if XYSet.mem (fst xy, snd xy + 1) set then (count + 1) else count in
    if XYSet.mem (fst xy + 1, snd xy + 1) set then (count + 1) else count in
  if neighbor_count set xy < 4 then true else false
;;

let rec solve list set out =
  let rec s list set out out_set =
    match list with
    | x :: y ->
      if is_grabbeble set x then s y set (out + 1) (XYSet.remove x out_set) else s y set out out_set
    | [] -> (out, out_set)
  in let (o, out_set) = s list set out set in
  if o > out then
    solve (XYSet.to_list out_set) out_set o
  else
    out
;;

let solve ic =
  let set = parse ic in
  solve (XYSet.to_list set) set 0
;;
