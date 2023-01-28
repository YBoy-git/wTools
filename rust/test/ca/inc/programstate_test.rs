use super::*;
use wtools::BasicError;
use wca::ProgramState;

//

tests_impls!
{
  fn set_and_get_pos()
  {
    let mut ps = ProgramState::default();
    a_id!( 0, ps.get_pos() );

    ps.set_pos( 1 );
    a_id!( 1, ps.get_pos() );

    ps.set_pos( usize::MAX );
    a_id!( usize::MAX, ps.get_pos() );
  }

  fn start_and_finish()
  {
    let mut ps = ProgramState::default();
    
    ps.finish();
    a_id!( usize::MAX, ps.get_pos() );

    ps.start();
    a_id!( usize::MIN, ps.get_pos() );
  }

  fn next_and_prev()
  {
    let mut ps = ProgramState::default();

    a_id!( 1, ps.next() );
    a_id!( Some( 1 ), ps.checked_next() );

    a_id!( 0, ps.prev() );
    a_id!( None, ps.checked_prev() );

    ps.finish();

    a_id!( usize::MAX, ps.next() );
    a_id!( None, ps.checked_next() );

    a_id!( usize::MAX - 1, ps.prev() );
    a_id!( Some( usize::MAX - 1 ), ps.checked_prev() );
  }

  fn go_next_and_go_prev()
  {
    let mut ps = ProgramState::default();
    
    ps.go_next();
    a_id!( 1, ps.get_pos() );
    ps.start();

    a_id!( Ok( () ), ps.checked_go_next() );
    a_id!( 1, ps.get_pos() );

    ps.go_back();
    a_id!( 0, ps.get_pos() );
    ps.start();

    a_id!( Err( BasicError::new( "It was the first instruction" ) ), ps.checked_go_back() );
    a_id!( 0, ps.get_pos() );

    ps.finish();

    ps.go_next();
    a_id!( usize::MAX, ps.get_pos() );

    a_id!( Err( BasicError::new( "It was the last instruction" ) ), ps.checked_go_next() );
    a_id!( usize::MAX, ps.get_pos() );

    ps.go_back();
    a_id!( usize::MAX - 1, ps.get_pos() );
    ps.finish();

    a_id!( Ok( () ), ps.checked_go_back() );
    a_id!( usize::MAX - 1, ps.get_pos() );
  }
}

//

tests_index!
{
  set_and_get_pos,
  start_and_finish,
  next_and_prev,
  go_next_and_go_prev,
}
