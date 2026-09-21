let calc x y = y - x + 1;;

let parse_range line =
  let (fst, snd) = Option.get (String.split_first ~sep:"-" line) in
  (int_of_string fst, int_of_string snd)
;;

let rec parse_ranges ic list =
  match String.trim (input_line ic) with
  | "" -> list
  | line -> parse_ranges ic ((parse_range line) :: list)
  | exception End_of_file -> list
;;

let merge_overlap list =
  let rec mo list list_out =
    match list with
    | x :: y :: z ->
      if (snd x) >= (snd y) then
        mo (x :: z) list_out
      else if (snd x) >= (fst y) then
        mo ((fst x, snd y) :: z) list_out
      else
        mo (y :: z) (x :: list_out)
    | [x] -> x :: list_out
    | [] -> list_out
  in List.rev (mo list [])
;;

let rec solve ranges out =
  match ranges with
  | x :: y -> solve y (out + (calc (fst x) (snd x)))
  | [] -> out
;;

let solve ic =
  let us_ranges = parse_ranges ic [] in
  let um_ranges = List.sort (fun a b -> Int.compare (fst a) (fst b)) us_ranges in
  let ranges = merge_overlap um_ranges in
  solve ranges 0
;;
