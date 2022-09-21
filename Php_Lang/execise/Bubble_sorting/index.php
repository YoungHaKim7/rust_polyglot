<?php
$arr = [ 6, 4, 7, 2, 9, 8, 1 ];
$length = count( $arr );
// Start by picking the first number and comparing it with the next one.
// That's starting with six and comparing it with the numbers that follow.

for( $outer = 0; $outer < $length; $outer++ ){
// and the first number immediately after it, starting with the last number at the end, the size of the game size.  for( $inner = $outer + 1; $inner < $length; $inner++ ){
  for( $inner = $outer +1; $inner < $length; $inner++) {
// If it's bigger than the last number, swap seats.
    if( $arr[ $outer ] > $arr[ $inner ] ){
      $temp = $arr[ $outer ];
      $arr[ $outer ] = $arr[ $inner ];
      $arr[ $inner ] = $temp;
    }
  }
}
print_r( $arr );
        ?>
