/*
  A map can be provided as a first argument whether to assign value or not
*/

function objectSetValue( objectDescriptor, field, value )
{
  if( objectDescriptor.skip === true )
  return;
  else
  objectDescriptor.src[ field ] = value;
}

module.exports = objectSetValue;
