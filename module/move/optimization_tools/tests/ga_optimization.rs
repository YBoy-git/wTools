use iter_tools::Itertools;
use optimization_tools::*;
use sudoku::*;
use optimization::*;
use deterministic_rand::{ Seed, Hrng };

mod tools;
use tools::*;

#[ test ]
fn crossover()
{
  logger_init();

  let board = Board::default();
  let hrng = Hrng::master_with_seed( Seed::default() );

  let parent1 = SudokuPerson::new( &board, hrng.clone() );
  log::trace!( "parent 1{parent1:#?}" );
  
  let parent2 = SudokuPerson::new( &board, hrng.clone() );
  log::trace!( "parent 2{parent2:#?}" );

  let operator = MultiplePointsBlockCrossover {};

  let child = operator.crossover( hrng.clone(), &parent1, &parent2 );
  log::trace!( "child {child:#?}" );
  let mut is_child = true;
  let mut has_first_parent_blocks = false;
  let mut has_second_parent_blocks = false;

  for i in child.board.blocks()
  {
    if child.board.block( i ).collect_vec() != parent1.board.block( i ).collect_vec() 
      && child.board.block( i ).collect_vec() != parent2.board.block( i ).collect_vec()
    {
      is_child = false;
    }

    if child.board.block( i ).collect_vec() == parent1.board.block( i ).collect_vec() 
    {
      has_first_parent_blocks = true;
    }

    if child.board.block( i ).collect_vec() == parent2.board.block( i ).collect_vec() 
    {
      has_second_parent_blocks = true;
    }
  }
  assert!( is_child && has_first_parent_blocks && has_second_parent_blocks );
}

/// Test GA on sudoku
///
/// # Usage
///
/// cargo test solve_with_ga --release --features rapidity_6
///
#[ cfg( feature = "rapidity_6" ) ]
#[ test ]
fn solve_with_ga()
{
  let sudoku : &str = r#"
  000042730
  308000024
  400360000
  006050840
  900403501
  500000070
  095006000
  000284956
  000005000
  "#;

  logger_init();
  log::set_max_level( log::LevelFilter::Warn );

  let mut initial = SudokuInitial::new_ga( Board::from( sudoku ), Seed::default() );

  let ( reason, generation ) = initial.solve_with_sa();

  log::trace!( "reason : {reason}" );
  a_true!( generation.is_some() );
  let generation = generation.unwrap();
  log::trace!( "{generation:#?}" );
  log::trace!( "{:#?}", generation.board );

  a_id!( generation.cost, 0.into() );

}

