//! Defines some internal helpers
//!
//! Main macros is the [`instruction`] macro.
//! This provides the ability to create a new instruction
//! in a short and readable way.
use crate::ParseError;
use arch::ArchError;

impl From<ArchError> for ParseError {
    fn from(value: ArchError) -> Self {
        Self::ArchError(value)
    }
}

#[macro_export]
/// Defines a new instruction or table of instructions
///
/// ## Usage
///
/// ```ignore
/// instruction!{
///     size 32; SomeTableIdent contains
///         SomeInstructinIdent : {
///              some_field_name as intermediateType (u8) : SomeFinalType : {start_bit} -> {end_bit} optional_conversion_method (try_into),
///         },
///         PossiblyMoreInstructions : .....
///         
///     }
/// };
/// ```
/// This macro invocation provides an enum SomeTableIdent containing the variants
/// (SomeInstructinIdent,PossiblyMoreInstructions) which in turn are structs
/// containing the fields defined in the { } block. All of the fields in SomeTableIdent
/// implement [`Parse`](crate::Parse).
macro_rules! instruction {
    (size $size:ty;
     $(
         $id:ident : {
            $(
                $field_id:ident $(as $representation:ty)? : $type:ty : $start:literal -> $end:literal $($expr:ident)?
            ),*
        }
    ),*
    ) => {
        $(
            paste!{
                #[doc = "Instruction " [<$id>] "\n\n"]
                #[doc = "Contains the following fields:\n"]
                $(
                    #[doc = "- " [<$field_id>] " of type " [<$type>] " from bit " [<$start>] " to bit " [<$end>] "\n"]
                )+
                #[derive(Debug)]
                pub struct $id {
                    $(
                        #[doc = "bit " [<$start>] " to " [<$end>]]
                        pub $field_id:$type,
                    )+
                }
            }


            impl Parse for $id{
                type Target = Self;
                // #[allow(unused_variables)]
                fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, crate::ParseError>
                where
                    Self: Sized {
                    let word: $size = match iter.consume::<1>(){
                        Some(buff) => Ok(buff[0]),
                        None => Err(ParseError::Invalid16Bit(stringify!($id))),
                    }?;
                    $(
                        let $field_id:$type = instruction!($size;word $(as $representation)?; $start -> $end $($expr)?);

                    )+
                    Ok(Self{
                        $(
                            $field_id: $field_id,
                        )+
                    })
                }
            }
        )+
    };

    (
        $size:ty; $word:ident $(as $representation:ty)?; $start:literal -> $end:literal $($expr:ident)?
    ) => {
            {
                #[allow(dead_code)]
                fn map<T:Into<ParseError>>(el:T) -> ParseError{
                    el.into()
                }
                (($word as $size).mask::<$start,$end>() $(as $representation)?)$(.$expr().map_err(|e| map(e))?)?
            }
    };

    (
    size $size:ty; $table:ident contains
        $(
            $($id:ident : {
                $(

                        $field_id:ident $(as $representation:ty)?: $type:ty : $start:literal -> $end:literal $($expr:ident)?


                ),*
            })?
            $(
                -> $table_id:ident
            )?
        ),*
    ) => {
        paste!{
            #[derive(Debug)]
            pub enum $table{
                $(
                    $($id($id),)?
                    $(
                        #[doc = "Externally defined instruction or set of instructions [`"  [<$table_id>]  "`]"]
                        [<Subtable $table_id>]($table_id),
                    )?
                )+
            }
        }
        $(

            $(
                paste!{
                    #[doc = "Instruction " [<$id>] " from table " [<$table>] "\n\n"]
                    #[doc = "Contains the following fields:\n"]
                    $(
                        #[doc = "- " [<$field_id>] " of type " [<$type>] " from bit " [<$start>] " to bit " [<$end>] "\n"]
                    )*
                    #[derive(Debug)]
                    pub struct $id {
                        $(
                            #[doc = "bit " [<$start>] " to " [<$end>]]
                            pub $field_id:$type,
                        )*
                    }
                }


                impl Parse for $id{
                    type Target = Self;
                    #[allow(unused_variables)]
                    fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, crate::ParseError>
                    where
                        Self: Sized {
                        // Consume a word from the buffer
                        let word:$size = match iter.consume::<1>(){
                            Some(buff) => Ok(buff[0]),
                            None => Err(ParseError::Invalid16Bit(stringify!($id))),
                        }?;
                        $(
                            let $field_id:$type = instruction!($size; word $(as $representation)?; $start -> $end $($expr)?);
                        )*
                        let ret = Self{
                            $(
                                $field_id: $field_id,
                            )*
                        };
                        Ok(ret)
                    }
                }
            )?
        )*
    }
}
#[macro_export]
/// Combines a list of integer type values in to another integer.
///
/// ## Usage
///
/// ```ignore
/// let i: u8 = 1;
/// let imm2: u8 = 2;
/// let imm3: u8 = 4;
/// let res: u32 = combine!(i:imm2,2:imm3,3,u32);
/// assert_eq!(0b110100, res);
/// let zero = 0;
/// let res: u32 = combine!(i:zero,2,u32);
/// assert_eq!(0b100, res)
/// ```
macro_rules! combine {
    ($first_id:ident:$($id:expr,$size:literal):*,$ret_ty:ty) => {
        {

            let mut counter:usize = {
                $($size+)*0
            };
            let mut sum: $ret_ty = $first_id as $ret_ty << counter;
            #[allow(unused_assignments)]
            {
                $(
                    counter = counter - $size;
                    sum |= (($id as $ret_ty) << counter) as $ret_ty;
                )*
            }
            sum
        }
    };
}

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        let i: u8 = 1;
        let imm2: u8 = 2;
        let imm3: u8 = 4;
        let res: u32 = combine!(i:imm2,2:imm3,3,u32);
        assert_eq!(0b110100, res);
        let zero = 0;
        let res: u32 = combine!(i:zero,2,u32);
        assert_eq!(0b100, res)
    }
}
