#[warn(unused_imports)]
#[macro_use]
extern crate singlemap;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate pest;
#[macro_use]
extern crate pest_derive;
use proc_macro::TokenStream;
mod com;




///```
///#[autowired]
/// struct EntityObj {
/// pub key: String,
///}
/// static defsingle: Option<EntityObj> = None;
/// // 展开
/// // macro_rules! defsingle {
/// //     () => {
/// //         single_get!( "defsingle", EntityObj)
/// //     };
/// // }
/// 
/// #[test]
/// fn test_one() {
///    single_add!(
///        "defsingle",
///        Box::new(EntityObj {
///            key: "hhhhh".to_string()
///        })
///    );
///    println!("注入后获取值 :{:?}", defsingle!().key);
/// }
/// ```
#[allow(warnings)]
#[proc_macro_attribute]
pub fn autowired(_attr: TokenStream, _input: TokenStream) -> TokenStream {
    return com::ioc::impl_autowired(_attr,_input.clone());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


#[test]
fn test_one(){
    let sin = single_init!();
    single_add!("a".to_string(),Box::new("aaaa".to_string()));
    let straa = single_for!(sin,&"a".to_string(),String);
    println!("rustl:{:?}",straa);
}