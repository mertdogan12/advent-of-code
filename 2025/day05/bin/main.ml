let read_lines file_name proc =
  let ic = open_in file_name in
  print_endline (string_of_int (proc ic));
  close_in ic;;

read_lines "inp.txt" Aoc.solve;;
