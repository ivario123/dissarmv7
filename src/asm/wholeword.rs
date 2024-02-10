use super::Statement;
use crate::{Parse, ParseError};

// #[macro_export]
// macro_rules! instruction {
//     ($(
//         $id:ident : {
//             $(
//                 $field_id:ident : $type:ty : $start:literal -> $end:literal $($expr:ident)?
//
//             ),*
//         }
//     ),*
//     ) => {
//         $(
//             paste!{
//                 #[doc = "Half word instruction " [<$id>] "\n\n"]
//                 #[doc = "Contains the following fields:\n"]
//                 $(
//                     #[doc = "- " [<$field_id>] " of type " [<$type>] " from bit " [<$start>] " to bit " [<$end>] "\n"]
//                 )+
//                 pub struct $id {
//                     $(
//                         #[doc = "bit " [<$start>] " to " [<$end>]]
//                         pub $field_id:$type,
//                     )+
//                 }
//             }
//
//
//             impl Parse for $id{
//                 type Target = Self;
//                 fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, crate::ParseError>
//                 where
//                     Self: Sized {
//                     // Use step instead of peek as we want to destroy this information
//
//                     let first_byte = match iter.step() {
//                         Some(b) => Ok(b),
//                         None => Err(ParseError::Invalid16Bit(
//                             stringify!($id)
//
//                         )
//                         ),
//                     }?;
//
//                     let second_byte = match iter.step() {
//                         Some(b) => Ok(b),
//                         None => Err(ParseError::Invalid16Bit(stringify!($id))),
//                     }?;
//                     let word = (first_byte as u16) << 8 | (second_byte as u16);
//                     $(
//                         let $field_id:$type = instruction!(word as $type; $start -> $end $($expr)?);
//
//                     )+
//                     Ok(Self{
//                         $(
//                             $field_id: $field_id,
//                         )+
//                     })
//                 }
//             }
//         )+
//     };
//     (
//         $word:ident as $type:ty; $start:literal -> $end:literal $($expr:ident)?
//     ) => {
//
//             (mask::<$start,$end>($word) as u8)$(.$expr()?)?
//         //$map_err(|e| crate::ParseError::InvalidField(format!("{:?}",e)))?
//     };
//     (
//     table $table:ident contains
//     $(
//         $id:ident : {
//             $(
//                 $field_id:ident : $type:ty : $start:literal -> $end:literal $($expr:ident)?
//
//             ),*
//         }
//     ),*) => {
//         paste!{
//             pub enum $table{
//                 $(
//                     $id($id),
//                 )+
//             }
//         }
//         $(
//             paste!{
//                 #[doc = "Half word instruction " [<$id>] " from table " [<$table>] "\n\n"]
//                 #[doc = "Contains the following fields:\n"]
//                 $(
//                     #[doc = "- " [<$field_id>] " of type " [<$type>] " from bit " [<$start>] " to bit " [<$end>] "\n"]
//                 )+
//                 #[derive(Debug)]
//                 pub struct $id {
//                     $(
//                         #[doc = "bit " [<$start>] " to " [<$end>]]
//                         pub $field_id:$type,
//                     )+
//                 }
//             }
//
//
//             impl Parse for $id{
//                 type Target = Self;
//                 fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, crate::ParseError>
//                 where
//                     Self: Sized {
//                     // Use step instead of peek as we want to destroy this information
//                     let first_byte = match iter.step() {
//                         Some(b) => Ok(b),
//                         None => Err(ParseError::Invalid16Bit(stringify!($id))),
//                     }?;
//
//                     let second_byte = match iter.step() {
//                         Some(b) => Ok(b),
//                         None => Err(ParseError::Invalid16Bit(stringify!($id))),
//                     }?;
//                     let word = (first_byte as u16) << 8 | (second_byte as u16);
//                     $(
//                         let $field_id:$type = instruction!(word as $type; $start -> $end $($expr)?);
//
//                     )+
//                     let ret = Self{
//                         $(
//                             $field_id: $field_id,
//                         )+
//                     };
//                     println!("Parsed {:?}",ret);
//                     Ok(ret)
//                 }
//             }
//         )*
//     }
// }
#[inline(always)]
fn mask<const START: usize, const END: usize>(num: u16) -> u16 {
    let intermediate = num >> START;
    let mask = ((1 << (END - START) as u16) as u16) - 1 as u16;

    let ret = intermediate & mask;
    // println!(
    //     "Masking {num:b} with mask {mask:b} from bit {START} to bit {END} resulting in {ret:b}"
    // );
    ret
}

/// A 16-bit wide instruction
pub trait FullWord: Statement {}

/// A 16-bit wide instruction
impl Parse for Box<dyn FullWord> {
    type Target = Box<dyn FullWord>;
    fn parse<T: crate::Stream>(iter: &mut T) -> Result<Self::Target, crate::ParseError> {
        let first: u16 = match iter.peek::<1>() {
            Some(value) => value,
            None => return Err(ParseError::IncompleteProgram),
        };
        let second: u16 = match iter.peek::<2>() {
            Some(value) => Ok(value),
            None => Err(ParseError::Inclomplete32Bit),
        }?;
        let op1 = mask::<10, 12>(first);
        let op2 = mask::<3, 10>(first);
        let op = mask::<14, 15>(second);
        println!("first : {:#016b}", first);
        println!("first : {:#016b}", second);
        println!("op1 {:#02b}", op1);
        println!("op2 {:#08b}", op2);
        println!("op {:#01b}", op);

        Err(ParseError::IncompleteProgram)
    }
}
