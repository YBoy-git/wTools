( function _l1_Entity_s_()
{

'use strict';

const _global = _global_;
const _ = _global_.wTools;
const Self = _.entity = _.entity || Object.create( null );

// --
// dichotomy
// --

/* xxx : adjust */
function is( src )
{
  return true;
}

//

function like( src )
{
  return _.entity.is( src );
}

// --
// exporter
// --

function strPrimitive( src )
{
  let result = '';

  _.assert( arguments.length === 1, 'Expects exactly one argument' );

  if( src === null || src === undefined )
  return;

  if( _.primitive.is( src ) )
  return String( src );

  return;
}

//

/**
 * Return primitive type of src.
 *
 * @example
 * let str = _.entity.strTypeSecondary( 'testing' );
 *
 * @param {*} src
 *
 * @return {string}
 * string name of type src
 * @function strTypeSecondary
 * @namespace Tools
 */

function strTypeSecondary( src )
{

  let name = Object.prototype.toString.call( src );
  let result = /\[(\w+) (\w+)\]/.exec( name );
  _.assert( !!result, 'unknown type', name );
  return result[ 2 ];
}

//

/**
 * Return type of src.
 *
 * @example
 * let str = _.entity.strType( 'testing' );
 *
 * @param {*} src
 *
 * @return {string}
 * string name of type src
 * @function strType
 * @namespace Tools
 */

/* qqq for Yevhen : jsdoc */
/* xxx : optimize later */
/* xxx : move to namesapce type? */
function strTypeWithTraits( src )
{

  _.assert( arguments.length === 1, 'Expects single argument' );

  if( _.aux.is( src ) )
  {

    if( _.mapIsPure( src ) )
    return 'Map.pure';
    else if( _.mapIsPolluted( src ) )
    return 'Map.polluted';
    else if( _.aux.isPure( src ) && _.aux.isPrototyped( src ) )
    return 'Aux.pure.prototyped';
    else if( _.aux.isPolluted( src ) && _.aux.isPrototyped( src ) )
    return 'Aux.polluted.prototyped';
    else _.assert( 0, 'undexpected' );

  }

  if( _.primitive.is( src ) )
  return end( _.entity.strTypeSecondary( src ) );

  let proto = Object.getPrototypeOf( src );
  if( proto && proto.constructor && proto.constructor !== Object && proto.constructor.name )
  return end( proto.constructor.name );

  return end( _.entity.strTypeSecondary( src ) );

  function end( result )
  {
    let translated = _.entity.TranslatedTypeMap[ result ];
    if( translated )
    result = translated;

    if( !_.entity.StandardTypeSet.has( result ) )
    {
      if( _.countableIs( src ) )
      result += '.countable';
      if( _.constructibleIs( src ) )
      result += '.constructible';
    }

    return result;
  }

}

//

/* qqq for Yevhen : jsdoc */
function strTypeWithoutTraits( src )
{

  _.assert( arguments.length === 1, 'Expects single argument' );

  if( _.aux.is( src ) )
  {

    if( _.mapIs( src ) )
    return 'Map';
    else
    return 'Aux';

  }

  if( _.primitive.is( src ) )
  return end( _.entity.strTypeSecondary( src ) );

  let proto = Object.getPrototypeOf( src );
  if( proto && proto.constructor && proto.constructor !== Object && proto.constructor.name )
  return end( proto.constructor.name );

  return end( _.entity.strTypeSecondary( src ) );

  function end( result )
  {
    let translated = _.entity.TranslatedTypeMap[ result ];
    if( translated )
    result = translated;
    return result;
  }

}

//

/* xxx : move out if-chain */
function exportStringShallowDiagnostic( src, o )
{
  _.assert( arguments.length === 1 || arguments.length === 2, 'Expects 1 or 2 arguments' );

  let result = '';

  if( _.primitive.is( src ) )
  {
    result = _.primitive.exportStringShallowDiagnostic( src );
  }
  else if( _.date.is( src ) )
  {
    result = _.date.exportStringShallowDiagnostic( src );
  }
  else if( _.regexpIs( src ) )
  {
    result = _.regexp.exportStringShallowDiagnostic( src );
  }
  else if( _.set.like( src ) )
  {
    result = _.set.exportStringShallowDiagnostic( src );
  }
  else if( _.hashMap.like( src ) )
  {
    result = _.hashMap.exportStringShallowDiagnostic( src );
  }
  else if( _.vector.like( src ) )
  {
    result = _.vector.exportStringShallowDiagnostic( src );
  }
  else if( _.routine.is( src ) )
  {
    result = _.routine.exportStringShallowDiagnostic( src );
  }
  else if( _.aux.like( src ) )
  {
    result = _.aux.exportStringShallowDiagnostic( src );
  }
  else if( _.object.like( src ) )
  {
    result = _.object.exportStringShallowDiagnostic( src );
  }
  else
  {
    result = String( src );
    result = _.strShort_( result ).result;
  }

  return result;
}

// --
// tools extension
// --

let ToolsExtension =
{

  entityIs : is.bind( _.entity ),
  entityLike : like.bind( _.entity ),

  strType : strTypeWithTraits,

  // lengthOf,
  // entity.lengthOf : lengthOf,

}

//

Object.assign( _, ToolsExtension );

// --
// entity extension
// --

let TranslatedTypeMap =
{

  'BigUint64Array' : 'U64x',
  'Uint32Array' : 'U32x',
  'Uint16Array' : 'U16x',
  'Uint8Array' : 'U8x',
  'Uint8ClampedArray' : 'U8ClampedX',

  'BigInt64Array' : 'I64x',
  'Int32Array' : 'I32x',
  'Int16Array' : 'I16x',
  'Int8Array' : 'I8x',

  'Float64Array' : 'F64x',
  'Float32Array' : 'F32x',

  'Buffer' : 'BufferNode',
  'ArrayBuffer' : 'BufferRaw',
  'SharedArrayBuffer' : 'BufferRawShared',
  'Map' : 'HashMap',
  'WeakMap' : 'HashMapWeak',
  'Function' : 'Routine',
  'Arguments' : 'ArgumentsArray',

}

let StandardTypeSet = new Set
([

  'U64x',
  'U32x',
  'U16x',
  'U8x',
  'U8ClampedX',
  'I64x',
  'I32x',
  'I16x',
  'I8x',
  'F64x',
  'F32x',

  'BufferNode',
  'BufferRaw',
  'BufferRawShared',
  'HashMap',
  'HashMapWeak',

  'ArgumentsArray',
  'Array',
  'Set',
  'Routine',
  'Global',

]);

let EntityExtension =
{

  // fields

  tools : _,
  TranslatedTypeMap,
  StandardTypeSet,

  // dichotmoy

  is,
  like,

  // exporter

  strPrimitive,
  strTypeSecondary,
  strType : strTypeWithTraits,
  strTypeWithTraits,
  strTypeWithoutTraits,
  // strParseType,
  // _strParseType,

  // exporter

  exportString : exportStringShallowDiagnostic,
  exportStringShallow : exportStringShallowDiagnostic,
  exportStringShallowDiagnostic,
  exportStringShallowCode : exportStringShallowDiagnostic,
  exportStringDiagnostic : exportStringShallowDiagnostic,
  exportStringCode : exportStringShallowDiagnostic,

  // lengthOf,

}

//

Object.assign( _.entity, EntityExtension );

})();