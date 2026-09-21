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

let rec exists i list =
  match list with
  | x :: y -> if i >= fst x && i <= snd x then true else exists i y
  | [] -> false
;;

let rec solve ic out ranges =
  match int_of_string (input_line ic) with
  | x ->
    if exists x ranges then
      solve ic (out + 1) ranges
    else
      solve ic out ranges
  | exception End_of_file -> out
;;

let solve ic =
  let us_ranges = parse_ranges ic [] in
  let ranges = List.sort (fun a b -> Int.compare (fst a) (fst b)) us_ranges in
  solve ic 0 ranges
;;
